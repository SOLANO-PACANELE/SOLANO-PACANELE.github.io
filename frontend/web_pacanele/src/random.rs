use core::f64;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use rand::thread_rng;

use rules::get_default_rule_set;

#[server]
pub async fn get_wheel_results(pcnl_count: u32) -> Result<(Vec<String>, u16), ServerFnError> {
    assert!(pcnl_count==3);

    let (result, reward) = rules::get_default_rule_set().play_random();
    info!("{result:?} => {reward}");

    Ok((result, reward))
}

#[cfg(feature = "server")]
async fn _wait_random(min_s: f64, max_s: f64) {
    use tokio::task::spawn_blocking;
    tokio::time::sleep(
        spawn_blocking(move || {
            let mut r = thread_rng();
            use rand::Rng;
            let wait = r.gen_range(min_s..max_s);
            let d = std::time::Duration::from_secs_f64(wait);
            d
        })
        .await
        .unwrap(),
    )
    .await;
}

#[server]
pub async fn get_wheel_shuffle(
    pcnl_id: u32,
    pcnl_count: u32,
) -> Result<Vec<String>, ServerFnError> {
    assert!(pcnl_count > 0 && pcnl_count < 6);
    assert!(pcnl_id < pcnl_count);
    Ok(srv_get_random_shuffle(pcnl_id, pcnl_count).await)
}

#[cfg(feature = "server")]
async fn srv_get_random_pcnl() -> String {
    let mut r = thread_rng();
    use rand::Rng;

    let f = crate::fruit_list::get_all_fruits();
    let c = r.gen::<usize>() % f.len();
    f[c].clone()
}

#[cfg(feature = "server")]
async fn srv_get_random_shuffle(pcnl_id: u32, pcnl_count: u32) -> Vec<String> {
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
    let mut v = crate::fruit_list::get_all_fruits().clone();
    v.shuffle(&mut r);
    v
}
