use rules::{get_default_rule_set, rule_set::RuleSet};
fn main() {
    let r = get_default_rule_set();
    
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