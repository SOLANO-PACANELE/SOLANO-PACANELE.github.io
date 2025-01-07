use rules::rule_set::RuleSet;

pub fn main () {
    
    let r =  RuleSet::random_rule_set(0.96);
    println!("{:#?}",r);
    let json = bincode::serialize(&r).unwrap();
    let mut file = std::fs::File::create("src/default_pacanea_rule_set.bin").unwrap();
    use std::io::Write;
    file.write_all(&json).unwrap();
}