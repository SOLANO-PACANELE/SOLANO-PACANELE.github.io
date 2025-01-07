use rules::rule_set::RuleSet;
fn main() {


    for i in 5..=15 {
        let i = i as f64 / 10.0;
        
        let p = RuleSet::random_rule_set(i).play_monte_carlo(100000);
        println!("I(dorit) = {} , P(primit) = {}", i, p);

    }
    let r =  RuleSet::random_rule_set(0.96);
    println!("{:?}",r);

    
    r.play_monte_carlo(10);
    r.play_monte_carlo(100);
    r.play_monte_carlo(1000);
    r.play_monte_carlo(10000);
    r.play_monte_carlo(100000);
    r.play_monte_carlo(1000000);
    r.play_monte_carlo(3000000);
    r.play_monte_carlo(6000000);
    r.play_monte_carlo(10000000);


}