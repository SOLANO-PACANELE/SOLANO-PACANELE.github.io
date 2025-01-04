use core::f64;
use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use rand::thread_rng;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Pacanele {},

}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

use once_cell::sync::Lazy;

static _ALL_FRUITS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./fruits.txt")
        .split_whitespace()
        .map(|s| s.trim().split(".").next().unwrap().to_string())
        .collect()
});

fn get_all_fruits() -> &'static Vec<String> {
    &_ALL_FRUITS
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { {make_animation_string(get_all_fruits().len() as u32)} }
        Router::<Route> {}
    }
}

fn make_animation_string(pic_count: u32) -> String {
    let mut css = "".to_string();
    css.push_str("@keyframes spin { ");
    const num_keyframes: u32 = 33;
    for index in 0..=num_keyframes {
        let rad = 2. * f64::consts::PI * index as f64 / num_keyframes as f64;
        let percent = index as f64 / num_keyframes as f64 * 100.0;
        let line_rule = make_transform_string(rad, pic_count);
        let line_css = format!("{percent}% {{ {line_rule} }}");
        css.push_str(&line_css);
    }
    css.push_str("}");
    css
}

fn make_transform_string(rad: f64, pic_count: u32) -> String {
    let size_coef = 2.9;
    let y = rad.sin() * size_coef * 100.0;
    let z = (rad.cos() - 1.5) * size_coef * 100.0;
    let z_index = ((rad.cos() - 2.0) * 100.0).round() as i32;
    let scale = 2.0 * f64::consts::PI / pic_count as f64 * 1.05 * size_coef;
    // info!("transform c={pic_count}   scale={scale}");
    format!(
        " transform:
    perspective(555cqmin)
     translate3d(0cqmin, {y}cqmin, {z}cqmin) 
     rotate3d(1, 0, 0, {rad}rad) scale3d({scale},{scale},{scale});

     z-index: {z_index};
    "
    )
}

#[derive(Debug, Clone, PartialEq)]
struct PcnlState {
    wheels: Vec<PcnlWheelState>,
}

#[derive(Debug, Clone, PartialEq)]
struct PcnlWheelState {
    pcnl_id: u32,
    pcnl_count: u32,
    new_fruit: String,
    old_fruit: String,
    spin_count: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct ShuffleState {
    wheels: Vec<WheelShuffleState>,
}

#[derive(Debug, Clone, PartialEq)]
struct WheelShuffleState {
    pcnl_id: u32,
    shuffle: Vec<String>,
    idx: HashMap<String, u32>,
}

#[component]
fn Pacanele() -> Element {
    info!("Paccanlee()");
    let pcnl_count: u32 = 3;

    let mut pcnl_state = use_signal(|| None);
    let mut shuf_state = use_signal(|| None);
    let mut can_spin = use_signal(|| false);
    let mut have_result = use_signal(|| false);
    let _init_state = use_resource(move || async move {
        info!("init_state");
        let mut v = vec![];
        let mut v2 = vec![];
        for i in 0..pcnl_count {
            v.push(PcnlWheelState {
                pcnl_id: i,
                pcnl_count,
                new_fruit: get_all_fruits()[0].clone(),
                old_fruit: get_all_fruits()[0].clone(),
                spin_count: 0,
            });
            let shuffle = get_wheel_shuffle(i, pcnl_count).await.unwrap();
            let shuf_idx = shuffle
                .iter()
                .enumerate()
                .map(|(i, x)| (x.clone(), i as u32))
                .collect::<HashMap<String, u32>>();
            v2.push(WheelShuffleState {
                pcnl_id: i,
                shuffle,
                idx: shuf_idx,
            });
        }
        pcnl_state.set(Some(PcnlState { wheels: v }));
        shuf_state.set(Some(ShuffleState { wheels: v2 }));
        can_spin.set(true);
        have_result.set(true);
        info!("init state done");
    });

    rsx! {
        div { id: "top-box",
            DebugSpinResult { pcnl_state }
        }
        div { id: "left-box" }
        div { id: "bottom-box" }
        div { id: "right-box",
            SpinButton {
                can_spin,
                have_result,
                pcnl_state,
                shuf_state,
                pcnl_count,
            }
        }

        div { id: "pacanele",
            div { id: "x777",
                SlotWheelRow { pcnl_state, shuf_state, pcnl_count , can_spin, have_result}

            }
        }
    }
}

#[component]
fn SlotWheelRow(
    pcnl_state: ReadOnlySignal<Option<PcnlState>>,
    shuf_state: ReadOnlySignal<Option<ShuffleState>>,
    pcnl_count: u32,
    can_spin: Signal<bool>,
    have_result: ReadOnlySignal<bool>,
) -> Element {
    info!("SlotWheelRow()");
    let has_spin = use_signal(move || {
        let mut v = vec![];
        for _i in 0..pcnl_count {
            v.push(false);
        }
        v
    });

    use_effect(move || {
        if *can_spin.peek() {
            return;
        }
        for _i in 0..pcnl_count {
            if has_spin.read()[_i as usize] {
                return;
            }
        }
        can_spin.set(true);
    });
    rsx! {
        for i in 0..pcnl_count {
            SlotWheelX {
                pcnl_id: i,
                data: pcnl_state,
                shuf_state,
                has_spin, have_result
            }
        }
    }
}

#[component]
fn SpinButton(
    can_spin: Signal<bool>,
    have_result: Signal<bool>,
    pcnl_state: Signal<Option<PcnlState>>,
    shuf_state: Signal<Option<ShuffleState>>,
    pcnl_count: u32,
) -> Element {
    info!("SpinButton()");

    let do_spin = move || async move {
        if *can_spin.peek() == false {
            info!("cannot spin.");
            return;
        }
        let Some(_shuf) = shuf_state.peek().as_ref() else {
            info!("shuf empty before spin");
            return;
        };
        let Some(mut state) = pcnl_state.peek().clone() else {
            info!("state empty before spin.");
            return;
        };
        can_spin.set(false);
        have_result.set(false);

        let Ok(new_results) = get_wheel_results(pcnl_count).await else {
            info!("server_wheel_resutls spin error");
            return;
        };
        assert!(new_results.len() == state.wheels.len());
        for (new_fruit, wheel) in new_results.into_iter().zip(state.wheels.iter_mut()) {
            wheel.spin_count += 1;
            wheel.old_fruit = wheel.new_fruit.clone();
            wheel.new_fruit = new_fruit;
        }
        pcnl_state.set(Some(state));
        have_result.set(true);
    };

    rsx! {
        {
            if *can_spin.read() {
                info!("spin button on");
                rsx! {
                    button {
                        onclick: move |_ev| {
                            let do_spin = do_spin.clone();
                            async move {
                                do_spin().await;
                            }
                        },
                        h1 { "Spin" }
                    }
                }
            } else {
                info!("spin button off");
                rsx! {}
            }
        }
    }
}

#[component]
fn SlotWheelX(
    pcnl_id: u32,
    data: ReadOnlySignal<Option<PcnlState>>,
    shuf_state: ReadOnlySignal<Option<ShuffleState>>,
    has_spin: Signal<Vec<bool>>,
    have_result: ReadOnlySignal<bool>,
) -> Element {
    info!("SlotWheelX({pcnl_id})");
    let mut state = use_signal(|| None);
    use_effect(move || {
        info!("SlotWheelXMemo({pcnl_id})");
        // let have_result = *have_result.read();
        if let (Some(data), Some(shuf)) = (data.read().as_ref(), shuf_state.read().as_ref()) {
            let _state = data.wheels[pcnl_id as usize].clone();
            let shuf = shuf.wheels[pcnl_id as usize].clone();
            assert_eq!(_state.pcnl_id, shuf.pcnl_id);

            let pic_count = shuf.shuffle.len() as u32;
            let idx_old = shuf.idx[&_state.old_fruit];
            let idx_new = shuf.idx[&_state.new_fruit];
            let slot_diff = (pic_count - idx_new + idx_old) % pic_count;
            let slot_diff = if slot_diff == 0 { pic_count } else { slot_diff };
            let rotations_diff = slot_diff as f64 / pic_count as f64;
            info!("rotations_diff: {rotations_diff}");
            // let rotations_diff = if have_result {rotations_diff} else {100.0};

            state.set(Some((_state, shuf, rotations_diff)));
        } else {
            state.set(None);
        }
    });
    let spin_period = 5.0;

    let _t = use_resource(move || async move {
        if let Some(_s) = state.read().as_ref() {
            if _s.0.spin_count > 0 {
                has_spin.write()[pcnl_id as usize] = true;
                use std::time::Duration;
                async_std::task::sleep(Duration::from_secs_f64(spin_period * _s.2)).await;
                has_spin.write()[pcnl_id as usize] = false;
            }
        }
    });

    let child_has_spin = use_memo(move || has_spin.read()[pcnl_id as usize]);

    rsx! {
        div { class: "slot-box",
            div { class: "slot-display",
                div { class: "pavaravan" }
                SlotWheelInner { pcnl_id, state, has_spin: child_has_spin , spin_period }
            }
        }
    }
}

#[component]
fn SlotWheelInner(
    pcnl_id: u32,
    state: ReadOnlySignal<Option<(PcnlWheelState, WheelShuffleState, f64)>>,
    has_spin: ReadOnlySignal<bool>,
    spin_period: f64,
) -> Element {
    if let Some((state, shuffle, _rotations_diff)) = state.read().as_ref() {
        let idx_old = shuffle.idx[&state.old_fruit];
        let idx_new = shuffle.idx[&state.new_fruit];
        rsx! {
            for (i , fruct) in shuffle.shuffle.iter().enumerate() {
                SlotImage {
                    pic_name: fruct.clone(),
                    pic_pos: i as u32,
                    pic_count: shuffle.shuffle.len() as u32,
                    spin_period,
                    idx_old,
                    idx_new,
                    spin_count: state.spin_count,
                    pcnl_id,
                    has_spin: *has_spin.read(),
                    _rotations_diff: *_rotations_diff,
                }
            }
        }
    } else {
        rsx! { "loading..." }
    }
}

#[component]
fn SlotImage(
    pic_name: String,
    pic_pos: u32,
    pic_count: u32,
    spin_period: f64,
    idx_old: u32,
    idx_new: u32,
    spin_count: u32,
    pcnl_id: u32,
    has_spin: bool,
    _rotations_diff: f64,
) -> Element {
    let img_id = format!("slot-img-{pcnl_id}-{pic_pos}-{spin_count}");
    let pic_pos_old = (pic_count + pic_pos - idx_old) % pic_count;
    let pic_pos_new = (pic_count + pic_pos - idx_new) % pic_count;
    let slot_diff = (pic_count + pic_pos_new - pic_pos_old) % pic_count;
    let slot_diff = if slot_diff == 0 { pic_count } else { slot_diff };
    let rotations_diff = slot_diff as f64 / pic_count as f64;
    let rotations_diff = _rotations_diff.max(rotations_diff);

    let spin_count = rotations_diff + pic_pos_old as f64 / pic_count as f64;
    let delay = spin_period * pic_pos_old as f64 / pic_count as f64;

    let rad_new = 2. * f64::consts::PI * pic_pos_new as f64 / pic_count as f64;
    let final_transform = make_transform_string(rad_new, pic_count);
    let animation = if has_spin {
        format!("animation: {spin_period}s linear {spin_count} -{delay}s spin ;")
    } else {
        "".to_string()
    };

    rsx! {
        img {
            id: img_id,
            class: "fruit-image",
            style: "{animation} {final_transform}",
            src: "/assets/img2/fruit/{pic_name}.png",
        }
    }
}

/// Echo the user input on the server.
#[server]
async fn get_wheel_results(pcnl_count: u32) -> Result<Vec<String>, ServerFnError> {
    assert!(pcnl_count > 0 && pcnl_count < 6);
    let mut res = vec![];
    res.reserve(pcnl_count as usize);
    for _i in 0..pcnl_count {
        res.push(srv_get_random_pcnl().await);
    }

    // simulate solana tranzaction delay
    wait_random(0.6, 5.5).await;

    Ok(res)
}

#[cfg(feature = "server")]
async fn wait_random(min_s: f64, max_s: f64) {
    use tokio::task::spawn_blocking;
    tokio::time::sleep(
        spawn_blocking(move || {
            let mut r = thread_rng();
            use rand::Rng;
            let wait = r.gen_range(min_s..max_s);
            let d = std::time::Duration::from_secs_f64(wait);
            d
        })
        .await.unwrap(),
    )
    .await;
}

#[server]
async fn get_wheel_shuffle(pcnl_id: u32, pcnl_count: u32) -> Result<Vec<String>, ServerFnError> {
    assert!(pcnl_count > 0 && pcnl_count < 6);
    assert!(pcnl_id < pcnl_count);
    Ok(srv_get_random_shuffle(pcnl_id, pcnl_count).await)
}

#[cfg(feature = "server")]
async fn srv_get_random_pcnl() -> String {
    let mut r = thread_rng();
    use rand::Rng;

    let c = r.gen::<usize>() % get_all_fruits().len();
    get_all_fruits()[c].clone()
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
    let mut v = get_all_fruits().clone();
    v.shuffle(&mut r);
    v
}

#[component]
fn DebugSpinResult(pcnl_state: ReadOnlySignal<Option<PcnlState>>) -> Element {
    let data = use_memo(move || {
        if let Some(data) = pcnl_state.read().as_ref() {
            let fruit = data
                .wheels
                .iter()
                .map(|w| {
                    w.new_fruit.clone()
                })
                .collect::<Vec<_>>();
            format!("{fruit:?}")
        } else {
            "loading...".to_string()
        }
    });
    rsx! {
        pre { {data} }
    }
}
