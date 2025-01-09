use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let fruits: Vec<_> = include_str!("src/fruits.txt")
        .trim()
        .split_whitespace()
        .map(|s| s.trim().to_string())
        .collect();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("get_all_fruits.rs");

    // let get_all_fruits = fn_get_all_fruits(&fruits);
    let enum_fruit = write_enum_fruit(&fruits);

    let all_code = format!("\n{enum_fruit}");

    fs::write(&dest_path, &all_code).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/fruits.txt");
}

fn write_enum_fruit(fruits: &[String]) -> String {
    let modified_fruit = fruits
        .iter()
        .map(|x| x.replace("-", "_"))
        .collect::<Vec<_>>();

    let mut variants_txt = "".to_string();
    let mut match_txt = "".to_string();
    for (i, (fruit, modified_fruit)) in fruits.iter().zip(modified_fruit.iter()).enumerate() {
        let line = format!("{modified_fruit} = {i},\n");
        variants_txt.push_str(&line);

        let line = format!("Self::{modified_fruit} => \"{fruit}\",\n");
        match_txt.push_str(&line);
    }
    format!(
        "
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy, 
        PartialEq, Eq, PartialOrd, Ord, Hash,
        strum::FromRepr, 
        serde::Serialize,
        serde::Deserialize, 
        strum_macros::EnumCount, 
        strum_macros::EnumIter, 
        strum::VariantArray,
    )]
    #[repr(u8)]
    pub enum Fruit {{
        {variants_txt}
    }}
    impl Fruit {{
        pub fn to_link_str(&self) -> &'static str {{
            match self {{
                {match_txt}
            }}
        }}
    }}
    "
    )
}

fn fn_get_all_fruits(fruits: &[String]) -> String {
    let fruit_count = fruits.len();
    let inner_str = fruits
        .into_iter()
        .map(|x| format!("\"{x}\""))
        .collect::<Vec<String>>()
        .join(",\n ");

    let part_a = format!("pub fn get_all_fruits() -> [&'static str;  {fruit_count}] {{");
    let part_b = format!("[{inner_str}]");
    let part_c = "}";
    format!("{part_a}{part_b}{part_c}")
}
