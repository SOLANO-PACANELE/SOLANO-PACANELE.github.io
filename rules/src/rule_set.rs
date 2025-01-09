use std::collections::BTreeMap;

use crate::Fruit;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RuleSet {
    pub(crate) prob: BTreeMap<Fruit, u16>,
    pub(crate) rewards: BTreeMap<(Fruit, u8), u16>,
    pub(crate) wheel_count: u8,
}

#[cfg(feature = "generate")]
pub fn make_random_prob_space(len: usize) -> Vec<u16> {
    assert!(len > 2);
    use rand::Rng;

    // pick random values but keep min,max
    let mut r = rand::thread_rng();
    // prob_var = how many times more probable one chip is as to another
    let prob_var = 4.0;
    let mut some_prob = vec![];
    for _ in 0..(len - 2) {
        some_prob.push(r.gen_range(1.0..prob_var));
    }
    some_prob.push(1.0);
    some_prob.push(prob_var * 1.2);

    // normalize
    let prob_sum: f64 = some_prob.iter().sum();
    for _p in some_prob.iter_mut() {
        *_p /= prob_sum;
        assert!(*_p > 0.0 && *_p < 1.0);
    }
    let mut some_prob: Vec<u16> = some_prob
        .into_iter()
        .map(|x| (x * std::u16::MAX as f64) as u16)
        .collect();
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

fn get_random_index_per_density(mut rand_val: u16, prob: &BTreeMap<Fruit, u16>) -> Fruit {
    assert!(prob.len() > 2);
    assert!(prob.len() < 200);
    use strum::VariantArray;

    for i in Fruit::all().iter() {
        if rand_val <= prob[i] {
            return *i;
        }
        rand_val -= prob[i];
    }
    return Fruit::VARIANTS[0];
}

/// get_prob(p, "banana", 1, 3) = prob that banana hits 1 time = p(banana, x, x) + p(x, banana, x) + p(x, x, banana), x != banana
///     p(x) = 1 - prob[fruit]
///     p("banana") = prob[fruit]
///  p(banana, 1, 3) = C(3, 1) * p(banana)^1 * p(x)^2
///  p(banana, 2, 3) = C(3, 2) * p(banana)^2 * p(x)^1
///  p(banana, 3, 3) = C(3, 3) * p(banana)^3 * p(x)^0
/// get_prob(p, "banana", 2, 3)
/// get_prob(p, "banana", 3, 3)
#[cfg(feature = "generate")]
fn get_prob_for_index_and_density(
    prob: &BTreeMap<Fruit, u16>,
    fruit: Fruit,
    count: u8,
    total: u8,
) -> f64 {
    assert!(count > 0);
    assert!(count <= total);
    assert!(total > 1);

    let p_fruit = prob[&fruit] as f64 / std::u16::MAX as f64;
    let p_x = 1.0 - p_fruit;

    let comb = combinari(total, count) as f64;

    comb * p_fruit.powi(count as i32) * p_x.powi(total as i32 - count as i32)
}

#[cfg(feature = "generate")]
fn factorial(n: u8) -> u64 {
    let mut t = 1_u64;
    for i in 1..=n {
        t *= i as u64;
    }
    t
}
#[cfg(feature = "generate")]
fn combinari(n: u8, k: u8) -> u64 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

impl RuleSet {
    #[cfg(feature = "generate")]
    pub fn play_monte_carlo(&self, count: u32) -> f64 {
        let mut reward_total: f64 = 0.;
        for _i in 0..count {
            reward_total += self.play_random().1 as f64;
        }
        let avg_reward = reward_total / count as f64;
        println!("avg_reward={avg_reward}  N={count}");
        avg_reward
    }
    #[cfg(feature = "generate")]
    pub fn play_random(&self) -> (Vec<Fruit>, u16) {
        use rand::Rng;

        let r: [u16; 3] = rand::thread_rng().gen();
        self.play_random_from_seed(r)
    }
    pub fn play_random_from_seed(&self, random_seed: [u16; 3]) -> (Vec<Fruit>, u16) {
        let mut result = vec![];
        let mut fruit_hits = BTreeMap::new();
        for i in 0..3 {
            let x = get_random_index_per_density(random_seed[i], &self.prob);
            result.push(x);
            let old_val = fruit_hits.get(&x);
            let new_val = if let Some(old_val) = old_val {
                old_val + 1
            } else {
                1
            };
            fruit_hits.insert(x, new_val);
        }

        // println!("hash = {fruit_hits:?}");
        let mut reward: u32 = 0;
        for (fruit, count) in fruit_hits.iter() {
            let rrr = *self.rewards.get(&(*fruit, *count)).unwrap_or(&0);
            reward += rrr as u32;
        }

        result.sort_by_key(move |x| -(fruit_hits[x] as i32));

        (result, reward.clamp(0, 55666) as u16)
    }
    #[cfg(feature = "generate")]
    pub fn projected_return(&self) -> f64 {
        let mut z = 0.0;

        for ((fruit, count), reward) in self.rewards.iter() {
            let prob = get_prob_for_index_and_density(&self.prob, *fruit, *count, self.wheel_count);
            let ev = prob * *reward as f64;
            z += ev;

            // println!("{fruit} x {count} ==> r={reward}  p={prob}  ev={ev}");
        }
        z
    }
    #[cfg(feature = "generate")]
    pub fn random_rule_set(desired_pay: f64) -> Self {
        use rand::Rng;

        assert!(desired_pay >= 0.5);
        assert!(desired_pay <= 2.0);

        let wheel_count = 3;

        let fruits = Fruit::all();
        let fruits_len_f64 = fruits.len() as f64;
        let prob = make_random_prob_space(fruits.len());
        let prob: BTreeMap<Fruit, u16> =
            BTreeMap::from_iter(fruits.iter().cloned().zip(prob.iter().cloned()));

        let mut rewards = BTreeMap::new();
        for fruit in fruits {
            // let p_fruit = prob[&fruit] as f64 / std::u16::MAX as f64;
            for score in 1..=wheel_count {
                let prob = get_prob_for_index_and_density(&prob, *fruit, score, wheel_count);
                assert!(prob > 0.0);
                assert!(prob < 1.0);
                let max_reward = (desired_pay / prob / fruits_len_f64).clamp(0.0, 55666.0);
                let max_reward_i = max_reward as u64;
                let r_u16 = max_reward_i.clamp(0, 55666) as u16;
                // println!("{fruit}x{score}   =>>>   reward_f: {max_reward}, reward_i: {max_reward_i}, reward_u16 = {r_u16}");
                rewards.insert((fruit.clone(), score), r_u16);
            }
        }

        // hardcode cherry
        rewards.insert((Fruit::cherry, 1), 1);

        for _ in 0..10 {
            let projected = RuleSet {
                prob: prob.clone(),
                rewards: rewards.clone(),
                wheel_count,
            }
            .projected_return();
            let coef = desired_pay / projected;
            for (_k, _v) in rewards.iter_mut() {
                if *_v > 3 {
                    *_v = ((*_v as f64) * coef * rand::thread_rng().gen_range(0.9999..1.0))
                        .floor()
                        .clamp(0.0, 55666.0) as u16;
                }
            }
            println!("projected: {projected}");
        }
        // filter rewards with 0 score
        let rewards = BTreeMap::from_iter(rewards.into_iter().filter(|k| k.1 > 0));
        RuleSet {
            prob,
            rewards,
            wheel_count,
        }
    }

    pub fn default_internal_deserialize() -> Self {
        let b = include_bytes!("default_pacanea_rule_set.bin");
        Self::deserialize(b)
    }

    pub fn prob(&self) -> BTreeMap<Fruit, u16> {
        self.prob.clone()
    }
    pub fn rewards(&self) -> BTreeMap<(Fruit, u8), u16> {
        self.rewards.clone()
    }

    pub fn serialize(&self) -> Vec<u8> {
        // borsh::to_vec(&self).unwrap()
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(v: &[u8]) -> Self {
        // borsh::from_slice(v).unwrap()
        bincode::deserialize(v).unwrap()
    }

    #[cfg(feature = "generate")]
    pub fn rust_constructor(&self, name: &str) -> String {
        let wheel_count = self.wheel_count;
        let prob_rows = self
            .prob
            .iter()
            .map(|(k, v)| format!("(Fruit::{k:?}, {v})"))
            .collect::<Vec<_>>()
            .join(",\n");

        let reward_rows = self
            .rewards
            .iter()
            .map(|((k, j), m)| format!("((Fruit::{k:?}, {j}), {m})"))
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            "
        use crate::Fruit;
        use crate::rule_set::RuleSet;
        use std::collections::BTreeMap;
        impl RuleSet {{
            pub fn {name}() -> Self {{
                Self {{
                    prob: BTreeMap::<Fruit, u16>::from([{prob_rows}]),
                    rewards: BTreeMap::<(Fruit, u8), u16>::from([{reward_rows}]),
                    wheel_count: {wheel_count},
                }}
            }}
        }}
        "
        )
    }
}
