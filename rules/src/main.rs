use rules::rule_set::RuleSet;
fn main() {
    let r = RuleSet::p96();
    let mut p_v: Vec<_> = r.prob().into_iter().collect();
    p_v.sort_by_key(|k| k.1);
    println!("prob: {:?}", p_v);

    println!("SEED [0,0,0]: {:?}", r.play_random_from_seed([0, 0, 0]));
    println!("PROJECTED RETURN: {}", r.projected_return());

    assert!(r.play_random_from_seed([0, 0, 0]) == r.play_random_from_seed([0, 0, 0]));

    r.play_monte_carlo(10000);
    r.play_monte_carlo(30000);
    r.play_monte_carlo(100000);
    r.play_monte_carlo(300000);
    r.play_monte_carlo(1000000);
}
