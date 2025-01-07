use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let fruits: Vec<_> = include_str!("src/fruits.txt").trim()
    .split_whitespace()
    .map(|s| s.trim().to_string())
    .collect();
    let fruit_count = fruits.len();
    let inner_str = fruits.into_iter().map(|x| format!("\"{x}\"")).collect::<Vec<String>>().join(", ");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("get_all_fruits.rs");
    let part_a = format!("pub fn get_all_fruits() -> [&'static str;  {fruit_count}] {{");
    let part_b = format!("[{inner_str}]");
    let part_c = "}";

    fs::write(
        &dest_path,
        format!("{part_a}{part_b}{part_c}")
    ).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/fruits.txt");
}