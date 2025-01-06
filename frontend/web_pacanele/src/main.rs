use core::f64;
use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use web_pacanele::{
    audio::{make_audio_loop_coroutine, send_audio_event, AudioEvent},
    fruit_list::get_all_fruits,
    gen_css::{make_animation_string, make_transform_string},
    random::{get_wheel_results, get_wheel_shuffle},
    state::{PcnlState, PcnlWheelState, ShuffleState, WheelShuffleState, WheelStage},
    time::{get_current_ts, sleep},
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Pacanele {},

}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { {make_animation_string("spin_1", get_all_fruits().len() as u32)} }
        document::Style { {make_animation_string("spin_2", get_all_fruits().len() as u32)} }
        Router::<Route> {}
    }
}

fn random_spin_period() -> f64 {
    let mut r = rand::thread_rng();
    use rand::Rng;
    r.gen_range(1.5..2.3)
}

#[component]
fn Pacanele() -> Element {
    info!("Paccanlee()");
    make_audio_loop_coroutine();

    let pcnl_count: u32 = 3;

    let mut pcnl_state = use_signal(|| None);
    let mut shuf_state = use_signal(|| None);
    let _init_state = use_resource(move || async move {
        info!("init_state");
        let mut v = vec![];
        let mut v2 = vec![];
        for i in 0..pcnl_count {
            let shuffle = get_wheel_shuffle(i, pcnl_count).await.unwrap();
            let shuf_idx = shuffle
                .iter()
                .enumerate()
                .map(|(i, x)| (x.clone(), i as u32))
                .collect::<HashMap<String, u32>>();

            let init_fruit = get_all_fruits()[0].clone();
            let init_idx = shuf_idx[&init_fruit];
            v2.push(WheelShuffleState {
                pcnl_id: i,
                shuffle,
                idx: shuf_idx,
            });

            v.push(PcnlWheelState {
                pcnl_id: i,
                pcnl_count,
                new_fruit: init_fruit.clone(),
                old_fruit: init_fruit.clone(),
                spin_count: 0,
                new_idx: init_idx,
                old_idx: init_idx,
                spin_period: random_spin_period(),
                wheel_stage: WheelStage::Ready,
                rotations_diff: 0.0,
            });
        }
        pcnl_state.set(Some(PcnlState { wheels: v }));
        shuf_state.set(Some(ShuffleState { wheels: v2 }));
        info!("init state done");
    });

    rsx! {
        div { id: "top-box",
            DebugSpinResult { pcnl_state }
        }
        div { id: "left-box" }
        div { id: "bottom-box" }
        div { id: "right-box",
            SpinButton { pcnl_state, shuf_state, pcnl_count }
        }

        div { id: "pacanele",
            div { id: "x777",
                SlotWheelRow { pcnl_state, shuf_state, pcnl_count }

            }
        }
    }
}

#[component]
fn SpinButton(
    pcnl_state: Signal<Option<PcnlState>>,
    shuf_state: Signal<Option<ShuffleState>>,
    pcnl_count: u32,
) -> Element {
    info!("SpinButton()");

    let mut effects_running = use_signal(|| false);
    let wheels_ready = use_memo(move || {
        if let Some(PcnlState { wheels }) = pcnl_state.read().as_ref() {
            for w in wheels.iter() {
                if w.wheel_stage != WheelStage::Ready {
                    return false;
                }
            }
            true
        } else {
            false
        }
    });

    // Do whole spin sequence while locking the "spin" button
    let do_spin = move || async move {
        if *wheels_ready.peek() == false {
            info!("cannot spin.");
            return;
        }
        let Some(_shuf) = shuf_state.peek().clone() else {
            info!("shuf empty before spin");
            return;
        };
        let Some(mut state) = pcnl_state.peek().clone() else {
            info!("state empty before spin.");
            return;
        };
        // Start spin. we do not yet have spin results (can take 5-10s on chain),
        // so we spin in place from the starting position a whole (integer) number of spins.
        let spin_time = get_current_ts();
        effects_running.set(true);
        for w in state.wheels.iter_mut() {
            w.wheel_stage = WheelStage::PendingResults;
            w.spin_period = random_spin_period();
            // also set old fruit to the new one, to have correct spin
            w.old_fruit = w.new_fruit.clone();
            w.old_idx = w.new_idx;
        }
        pcnl_state.set(Some(state.clone()));
        send_audio_event(AudioEvent::StartSpin);

        let Ok(new_results) = get_wheel_results(pcnl_count).await else {
            info!("server_wheel_resutls spin error");
            return;
        };
        assert!(new_results.len() == state.wheels.len());
        send_audio_event(AudioEvent::HaveResults);

        // now that we have the results, we can diverge into each wheel
        let mut _fut = vec![];
        for seq in compute_wheel_sequences(&state, &_shuf, new_results, spin_time) {
            _fut.push(spawn(async move {
                sleep(seq.first_wait).await;
                if let Some(x) = pcnl_state.write().as_mut() {
                    x.wheels[seq.pcnl_id as usize] = seq.first_val;
                }
                sleep(seq.second_wait).await;
                if let Some(x) = pcnl_state.write().as_mut() {
                    x.wheels[seq.pcnl_id as usize] = seq.second_val;
                }
                send_audio_event(AudioEvent::WheelStop {
                    wheel_id: seq.pcnl_id,
                    pcnl_count
                });
            }));
        }

        // wait until all pcnl is ready, then send audio stop events
        spawn(async move {
            while !*wheels_ready.peek() {
                sleep(0.1).await;
            }
            sleep(0.4).await;
            send_audio_event(AudioEvent::WheelsFinished);
            sleep(0.1).await;
            send_audio_event(AudioEvent::StopAudio);
            sleep(0.1).await;
            effects_running.set(false);
        });
    };

    rsx! {
        {
            if *wheels_ready.read() && !*effects_running.read() {
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

struct WheelSequenceInfo {
    pcnl_id: u32,
    first_wait: f64,
    first_val: PcnlWheelState,
    second_wait: f64,
    second_val: PcnlWheelState,
}

fn compute_wheel_sequences(
    state: &PcnlState,
    shuf: &ShuffleState,
    new_results: Vec<String>,
    spin_time: f64,
) -> Vec<WheelSequenceInfo> {
    let mut v: Vec<WheelSequenceInfo> = new_results
        .into_iter()
        .zip(state.wheels.iter().zip(shuf.wheels.iter().cloned()))
        .enumerate()
        .map(|(pcnl_id, (new_fruit, (wheel, w_shuf)))| {
            let mut wheel = wheel.clone();
            let pcnl_id = pcnl_id as u32;

            // wait until whole number of spins passes
            let spin_period = wheel.spin_period;
            let elapsed = get_current_ts() - spin_time;
            let elapsed_periods = elapsed / spin_period;
            let remaining_period = 1.0 - elapsed_periods.fract();
            let remaining_secs = remaining_period * spin_period;
            let first_wait = remaining_secs;

            // update the results after whole number of spins
            wheel.spin_count += 1;
            wheel.old_fruit = wheel.new_fruit.clone();
            wheel.new_fruit = new_fruit;
            wheel.old_idx = w_shuf.idx[&wheel.old_fruit];
            wheel.new_idx = w_shuf.idx[&wheel.new_fruit];
            wheel.wheel_stage = WheelStage::HaveResults;

            let pic_count = w_shuf.shuffle.len() as u32;
            let slot_diff = (pic_count - wheel.new_idx + wheel.old_idx) % pic_count;
            let slot_diff = if slot_diff == 0 { pic_count } else { slot_diff };
            let rotations_diff = slot_diff as f64 / pic_count as f64;

            wheel.rotations_diff = rotations_diff;

            let first_val = wheel.clone();

            // wait until last part of spin finishes, then set ready
            let second_wait = rotations_diff * spin_period;
            wheel.wheel_stage = WheelStage::Ready;
            let second_val = wheel;

            WheelSequenceInfo {
                first_val,
                first_wait,
                second_val,
                second_wait,
                pcnl_id,
            }
        })
        .collect();

    // wheels stop left to right, staggered at least 0.1s
    for pcnl_id in 1..state.wheels.len() {
        let duration_prev = v[pcnl_id-1].first_wait + v[pcnl_id-1].second_wait;
        while v[pcnl_id].first_wait + v[pcnl_id].second_wait < 0.2 + duration_prev {
            v[pcnl_id].first_wait += v[pcnl_id].first_val.spin_period;
        }
    }
    v
}

#[component]
fn SlotWheelRow(
    pcnl_state: ReadOnlySignal<Option<PcnlState>>,
    shuf_state: ReadOnlySignal<Option<ShuffleState>>,
    pcnl_count: u32,
) -> Element {
    info!("SlotWheelRow()");
    rsx! {
        for i in 0..pcnl_count {
            SlotWheelX { pcnl_id: i, pcnl_state, shuf_state }
        }
    }
}

#[component]
fn SlotWheelX(
    pcnl_id: u32,
    pcnl_state: ReadOnlySignal<Option<PcnlState>>,
    shuf_state: ReadOnlySignal<Option<ShuffleState>>,
) -> Element {
    info!("SlotWheelX({pcnl_id})");
    let mut state = use_signal(|| None);
    use_effect(move || {
        info!("SlotWheelXMemo({pcnl_id})");
        // let have_result = *have_result.read();
        if let (Some(data), Some(shuf)) = (pcnl_state.read().as_ref(), shuf_state.read().as_ref()) {
            let _state = data.wheels[pcnl_id as usize].clone();
            let shuf = shuf.wheels[pcnl_id as usize].clone();
            assert_eq!(_state.pcnl_id, shuf.pcnl_id);

            state.set(Some((_state, shuf)));
        } else {
            state.set(None);
        }
    });

    rsx! {
        div { class: "slot-box",
            div { class: "slot-display",
                div { class: "pavaravan" }
                div { class: "pavaravan2" }
                div { class: "pavaravan3" }
                div { class: "line-marker" }
                SlotWheelInner { state }
            }
        }
    }
}

#[component]
fn SlotWheelInner(state: ReadOnlySignal<Option<(PcnlWheelState, WheelShuffleState)>>) -> Element {
    if let Some((state, shuffle)) = state.read().as_ref() {
        rsx! {
            for (i , fruct) in shuffle.shuffle.iter().enumerate() {
                SlotImage {
                    pic_name: fruct.clone(),
                    pic_pos: i as u32,
                    pic_count: shuffle.shuffle.len() as u32,
                    state: state.clone(),
                }
            }
        }
    } else {
        rsx! { "loading..." }
    }
}

#[component]
fn SlotImage(pic_name: String, pic_pos: u32, pic_count: u32, state: PcnlWheelState) -> Element {
    let pic_pos_old = (pic_count + pic_pos - state.old_idx) % pic_count;
    let pic_pos_new = (pic_count + pic_pos - state.new_idx) % pic_count;
    let slot_diff = (pic_count + pic_pos_new - pic_pos_old) % pic_count;
    let slot_diff = if slot_diff == 0 { pic_count } else { slot_diff };
    let rotations_diff = slot_diff as f64 / pic_count as f64;
    let rotations_diff = state.rotations_diff.max(rotations_diff);
    let spin_period = state.spin_period;

    let spin_count = rotations_diff + pic_pos_old as f64 / pic_count as f64;
    let delay = spin_period * pic_pos_old as f64 / pic_count as f64;

    let rad_new = 2. * f64::consts::PI * pic_pos_new as f64 / pic_count as f64;
    let final_transform = make_transform_string(rad_new, pic_count);
    let animation = match state.wheel_stage {
        WheelStage::Ready => "".to_string(),
        WheelStage::PendingResults => {
            format!("animation: {spin_period}s linear infinite -{delay}s spin_1 ;")
        }
        WheelStage::HaveResults => {
            format!("animation: {spin_period}s linear {spin_count} -{delay}s spin_2 ;")
        }
    };

    rsx! {
        img {
            class: "fruit-image",
            style: "{animation} {final_transform}",
            src: "/assets/img2/fruit/{pic_name}.png",
        }
    }
}

#[component]
fn DebugSpinResult(pcnl_state: ReadOnlySignal<Option<PcnlState>>) -> Element {
    let data = use_memo(move || {
        if let Some(data) = pcnl_state.read().as_ref() {
            let fruit = data
                .wheels
                .iter()
                .map(|w| (w.new_fruit.clone(), w.wheel_stage.clone()))
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
