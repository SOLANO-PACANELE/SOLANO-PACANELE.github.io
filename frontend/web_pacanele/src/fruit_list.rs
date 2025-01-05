use once_cell::sync::Lazy;

static _ALL_FRUITS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./fruits.txt")
        .split_whitespace()
        .map(|s| s.trim().split(".").next().unwrap().to_string())
        .collect()
});

pub fn get_all_fruits() -> &'static Vec<String> {
    &_ALL_FRUITS
}
