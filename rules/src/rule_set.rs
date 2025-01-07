use std::collections::HashMap;

use rand::Rng;

use crate::get_all_fruits;

pub(crate) struct RuleSet {
    prob: HashMap<&'static str, f64>,
    reward: HashMap<(&'static str, u8), u16>,
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

impl RuleSet {
    // fn random() -> Self {
    //     let fruit = get_all_fruits();


        

    //     RuleSet { prob, reward }
    // }
}