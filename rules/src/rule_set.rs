use std::collections::HashMap;

use rand::{thread_rng, Rng};

use crate::get_all_fruits;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSet {
    prob: HashMap<String, u16>,
    rewards: HashMap<(String, u8), u16>,
    wheel_count: u8,
}

pub fn random_prob_space(len: usize) -> Vec<u16> {
    assert!(len>2);

    // pick random values but keep min,max
    let mut r = rand::thread_rng();
    // prob_var = how many times more probable one chip is as to another
    let prob_var = 100.0;
    let mut some_prob = vec![];
    for _ in 0..(len-2) {
        some_prob.push(r.gen_range(1.0..prob_var));
    }
    some_prob.push(1.0);
    some_prob.push(prob_var);

    // normalize
    let prob_sum: f64 = some_prob.iter().sum();
    for _p in some_prob.iter_mut() {
        *_p /= prob_sum;
        assert!(*_p > 0.0 && *_p < 1.0);
    }
    let mut some_prob: Vec<u16> = some_prob.into_iter().map(|x| (x * std::u16::MAX as f64) as u16).collect();
    let pre_sum = std::u16::MAX - some_prob.iter().sum::<u16>();
    for _ in 0..pre_sum {
        let i = r.gen_range(0..some_prob.len());
        some_prob[i] += 1;
    }
    some_prob.sort();
    some_prob.reverse();

    assert_eq!(some_prob.iter().sum::<u16>(), std::u16::MAX);

    some_prob
}


fn get_random_index_per_density(mut rand_val: u16, prob: & HashMap<String, u16>) -> String {
    assert!(prob.len() > 2);
    assert!(prob.len() < 200);

    for i in prob.keys() {
        if rand_val <= prob[i] {
            return i.to_string();
        }
        rand_val -= prob[i];
    }
    return prob.keys().next().unwrap().to_string();
}
/// get_prob(p, "banana", 1, 3) = prob that banana hits 1 time = p(banana, x, x) + p(x, banana, x) + p(x, x, banana), x != banana
///     p(x) = 1 - prob[fruit]
///     p("banana") = prob[fruit]
///  p(banana, 1, 3) = C(3, 1) * p(banana)^1 * p(x)^2
///  p(banana, 2, 3) = C(3, 2) * p(banana)^2 * p(x)^1
///  p(banana, 3, 3) = C(3, 3) * p(banana)^3 * p(x)^0
/// get_prob(p, "banana", 2, 3)
/// get_prob(p, "banana", 3, 3)
fn get_prob_for_index_and_density( prob: & HashMap<String, u16>, fruit: &str, count: u8, total: u8) -> f64 {
    assert!(count > 0);
    assert!(count <= total);
    assert!(total > 1);

    let p_fruit = prob[fruit] as f64 / std::u16::MAX as f64;
    let p_x = 1.0 - p_fruit;

    let comb = combinari(total, count) as f64;

    comb * p_fruit.powi(count as i32) * p_x.powi(total as i32 - count as i32)
}


fn factorial(n: u8) -> u64 {
    let mut t = 1_u64;
    for i in 1..=n {
        t *= i as u64;
    }
    t
}

fn combinari(n: u8, k: u8) -> u64 {
    factorial(n) / (factorial(k) * factorial(n-k))
}

impl RuleSet {
    pub fn play_monte_carlo(&self, count: u32) -> f64 {
        let mut reward_total: f64 = 0.;
        for _i in 0..count {
            reward_total += self.play_random().1 as f64;
        }
        let avg_reward = reward_total / count as f64;
        println!("avg_reward={avg_reward}  N={count}");
        avg_reward
    }
    pub fn play_random(&self) -> (Vec<String>, u16) {
        let r: [u16; 3] = thread_rng().gen();
        self.play_random_from_seed(r)
    }
    pub fn play_random_from_seed(&self, random_seed: [u16;3]) -> (Vec<String>, u16) {
        let mut result = vec![];
        let mut fruit_hits = HashMap::new();
        for i in 0..3 {
            let x = get_random_index_per_density(random_seed[i], &self.prob);
            result.push(x.to_string());
            let old_val = fruit_hits.get(&x);
            let new_val = if let Some(old_val) = old_val {old_val+ 1} else {1};
            fruit_hits.insert(x, new_val);

        }

        // println!("hash = {fruit_hits:?}");
        let mut reward: u32 = 0;
        for (fruit, count) in fruit_hits.iter() {
            let rrr = *self.rewards.get(&(fruit.to_string(), *count)).unwrap_or(&0);
            reward += rrr as u32;
        }

        (result, reward.clamp(0, 55666) as u16)
    }
    pub fn random_rule_set(desired_pay: f64) -> Self {
        assert!(desired_pay >= 0.5);
        assert!(desired_pay <= 2.0);

        let wheel_count = 3;

        let fruits = get_all_fruits().map(|x| x.to_string());
        let prob = random_prob_space(fruits.len());
        let prob: HashMap<String, u16> = HashMap::from_iter(fruits.iter().cloned().zip(prob.iter().cloned()));

        let mut rewards = HashMap::new();
        for fruit in fruits {
            let p_fruit = prob[&fruit] as f64 / std::u16::MAX as f64;
            for score in 2..=wheel_count {
                let prob = get_prob_for_index_and_density(&prob, &fruit, score, wheel_count);
                assert!(prob > 0.0);
                assert!(prob < 1.0);
                let max_reward =( 1.0 / prob * p_fruit / 2.0 ).clamp(0.0, 55666.0);
                let max_reward_i = max_reward as u64;
                let r_u16 = max_reward_i.clamp(0, 55666) as u16;
                // println!("{fruit}x{score}   =>>>   reward_f: {max_reward}, reward_i: {max_reward_i}, reward_u16 = {r_u16}");
                rewards.insert((fruit.clone(), score), r_u16 );
            }
        }

        RuleSet { prob, rewards: rewards, wheel_count }
    }
}