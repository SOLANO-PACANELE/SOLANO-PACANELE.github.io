#[cfg(feature = "constructors")]
pub mod generated_rules;

pub mod rule_set;

include!(concat!(env!("OUT_DIR"), "/get_all_fruits.rs"));

impl Fruit {
    pub fn all() -> &'static [Self] {
        use strum::VariantArray;
        Fruit::VARIANTS
    }
}
