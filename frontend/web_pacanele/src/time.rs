// use dioxus_logger::tracing::info;

pub fn get_current_ts() -> f64 {
    web_time::SystemTime::now()
        .duration_since(web_time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

pub async fn sleep(secs: f64) {
    use std::time::Duration;
    let t0 = get_current_ts();
    async_std::task::sleep(Duration::from_secs_f64(secs)).await;
    let t1 = get_current_ts();
    // info!("sleep diff time: {} ms", (t1 - t0 - secs) * 1000.0);
}
