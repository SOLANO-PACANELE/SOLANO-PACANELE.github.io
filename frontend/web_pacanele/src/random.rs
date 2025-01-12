use dioxus::prelude::*;
use rules::Fruit;


pub fn get_wheel_shuffle(pcnl_id: u32, pcnl_count: u32) -> Vec<Fruit> {
    use rand::prelude::SliceRandom;
    let b = pcnl_id.to_le_bytes();
    let c = pcnl_count.to_le_bytes();
    let b1 = [[b, c]; 4];
    let b2 = b1.as_flattened().as_flattened();
    let mut b3: [u8; 32] = [0; 32];
    assert!(b2.len() == b3.len());
    for (i, (a, b)) in b2.iter().zip(b3.iter_mut()).enumerate() {
        let i = i as u8 ^ *a;
        *b = i;
    }

    let mut r = <rand_chacha::ChaCha20Rng as rand::SeedableRng>::from_seed(b3);
    let mut v = rules::Fruit::all().iter().cloned().collect::<Vec<_>>();
    v.shuffle(&mut r);
    v
}
