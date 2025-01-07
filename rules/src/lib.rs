use rule_set::RuleSet;

pub mod rule_set;

include!(concat!(env!("OUT_DIR"), "/get_all_fruits.rs"));

pub fn get_default_rule_set() -> RuleSet {
    let b=  include_bytes!("default_pacanea_rule_set.bin");
    let r: RuleSet = bincode::deserialize(b).unwrap();
    r
}