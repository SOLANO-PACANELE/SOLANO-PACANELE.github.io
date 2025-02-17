use core::f64;
use std::collections::HashMap;

use crate::{
    audio::{send_audio_event, AudioEvent},
    client::get_spin_result_from_solana,
    gen_css::make_transform_string,
    random::get_wheel_shuffle,
    state::{PcnlState, PcnlWheelState, ShuffleState, WheelShuffleState, WheelStage},
    time::{get_current_ts, sleep},
    wallet::{wallet_signals, BetAmountControl, CurrentWalletDropdown},
};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use rules::{rule_set::RuleSet, Fruit};

fn random_spin_period(on_autoplay: bool) -> f64 {
    let mut r = rand::thread_rng();
    use rand::Rng;
    if on_autoplay {
        r.gen_range(0.4..0.7)
    } else {
        r.gen_range(0.9..1.6)
    }
}

#[component]
pub fn Pacanele() -> Element {
    let pcnl_count: u32 = 3;

    let mut pcnl_state = use_signal(|| None);
    let mut shuf_state = use_signal(|| None);
    let enable_autoplay = use_signal(|| false);
    let _init_state = use_resource(move || async move {
        let mut v = vec![];
        let mut v2 = vec![];
        for i in 0..pcnl_count {
            let shuffle = get_wheel_shuffle(i, pcnl_count);
            let shuf_idx = shuffle
                .iter()
                .enumerate()
                .map(|(i, x)| (*x, i as u32))
                .collect::<HashMap<Fruit, u32>>();

            let init_fruit = Fruit::all()[0];
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
                spin_period: random_spin_period(*enable_autoplay.peek()),
                wheel_stage: WheelStage::Ready,
                rotations_diff: 0.0,
            });
        }
        pcnl_state.set(Some(PcnlState {
            wheels: v,
            last_win: None,
            last_messages: vec![],
        }));
        shuf_state.set(Some(ShuffleState { wheels: v2 }));
    });

    rsx! {
        div { id: "top-box",
            // DebugSpinResult { pcnl_state }
            Win {pcnl_state}
        }
        div { id: "left-box" ,
            DisplayCredit {}
            DisplayWinCombo {}
        }
        div {
            id: "bottom-box",
            Autoplay{enable_autoplay},
            CurrentWalletDropdown{}
        }
        div { id: "right-box",
            SpinButton { pcnl_state, shuf_state, pcnl_count, enable_autoplay }
            
            BetAmountControl {}
        }

        div { id: "pacanele",
            div { id: "x777",
                SlotWheelRow { pcnl_state, shuf_state, pcnl_count }

            }
        }
    }
}

#[component]
fn DisplayCredit() -> Element {
    let wallet = wallet_signals();

    rsx! {
        h1 {
            style: "font-size: 400%",
            "credit: {wallet.current_credit}"
        }
    }
}

#[component]
fn DisplayWinCombo() -> Element {
    let r = RuleSet::default_internal_deserialize().rewards();
    let mut r = r
        .iter()
        .filter(|x| *x.1 > 0)
        .map(|x| ((x.0 .0.to_link_str(), x.0 .1), *x.1))
        .collect::<Vec<_>>();
    r.sort_by_key(|a| -(a.1 as i32) - a.0 .1 as i32);

    rsx! {
        div {
            class: "display-win-combo",
            for ((fruit, count), reward) in r {
                DisplayWinSingleCombo {fruit, count, reward}
            }
        }
    }
}
#[component]
fn DisplayWinSingleCombo(fruit: String, count: u8, reward: u16) -> Element {
    rsx! {
        div {
            class: "display-win-combo-single",
            for _i in 0..count {
                img {
                    class: "combo-image",
                    src: "/assets/img2/fruit/{fruit}.png",
                }
            }
            {format!("{reward}")}
        }
    }
}

#[component]
pub fn Autoplay(enable_autoplay: Signal<bool>) -> Element {
    let mut name = use_signal(|| "false".to_string());

    use_effect(move || {
        let n = name.read().clone();
        if n == "true" {
            enable_autoplay.set(true);
        } else {
            enable_autoplay.set(false);
        }
    });

    rsx! {
        h1 {
            style: "display: flex; align-items: center;",
            "Autoplay",
            input {
                r#type: "checkbox",
                style: "width: 40pt; height: 40pt;",
                // we tell the component what to render
                value: "{name}",
                // and what to do when the value changes
                oninput: move |event| name.set(event.value())
            }
        }
    }
}

#[component]
fn Win(pcnl_state: Signal<Option<PcnlState>>) -> Element {
    let win_box = if let Some(r) = pcnl_state.read().as_ref() {
        if let Some(w) = r.last_win {
            rsx! {
                h1 {
                    style:"font-size:400%;color:red;",
                    "Win: " {format!("{}", w)}
                }
            }
        } else {
            rsx! {}
        }
    } else {
        rsx! {}
    };

    let msg_box = if let Some(r) = pcnl_state.read().as_ref() {
        let s = r.last_messages.join("\n");
        rsx! {pre {"{s}"}}
    } else {
        rsx! {}
    };

    rsx! {
        div {
            style: "display: flex",
            div {
                style:"border: 1px solid red; width: 50cqw; height: 100cqh;",
                {win_box}
            }
            div {
                style:"border: 1px solid red; width: 50cqw; height: 100cqh;",
                {msg_box}
            }

        }
    }
}

#[component]
fn SpinButton(
    pcnl_state: Signal<Option<PcnlState>>,
    shuf_state: Signal<Option<ShuffleState>>,
    pcnl_count: u32,
    enable_autoplay: ReadOnlySignal<bool>,
) -> Element {
    let wallet: crate::wallet::WalletSignals = wallet_signals();

    let mut effects_running = use_signal(|| false);
    let have_money = use_memo(move || {
        *wallet.current_bet_exp.read() > 10 && *wallet.current_credit.read() > 0
    });

    let wheels_ready = use_memo(move || {
        if let Some(PcnlState { wheels, .. }) = pcnl_state.read().as_ref() {
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

    let mut do_auto_respin = use_signal(|| false);
    let spin_courutine = use_coroutine(move |mut rx: UnboundedReceiver<()>| {
        async move {
            loop {
                use futures_util::stream::StreamExt;
                let _rx = rx.next().await;
                while let Ok(Some(_m)) = rx.try_next() {
                    // drop duplicate messages, we're already in.
                }
                if *wheels_ready.peek() == false {
                    info!("cannot spin.");
                    continue;
                }
                let Some(_shuf) = shuf_state.peek().clone() else {
                    info!("shuf empty before spin");
                    continue;
                };
                let Some(mut state) = pcnl_state.peek().clone() else {
                    info!("state empty before spin.");
                    continue;
                };
                if !*have_money.read() {
                    info!("no money.");
                    continue;
                }

                state.last_win = None;
                state.last_messages = vec![];
                let state_init = state.clone();
                // Start spin. we do not yet have spin results (can take 5-10s on chain),
                // so we spin in place from the starting position a whole (integer) number of spins.
                effects_running.set(true);

                do_auto_respin.set(false);
                for w in state.wheels.iter_mut() {
                    w.wheel_stage = WheelStage::PendingResults;
                    w.spin_period = random_spin_period(*enable_autoplay.peek());

                    // also set old fruit to the new one, to have correct spin
                    w.old_fruit = w.new_fruit.clone();
                    w.old_idx = w.new_idx;
                }
                pcnl_state.set(Some(state.clone()));
                let spin_time = get_current_ts();
                sleep(0.01).await;
                send_audio_event(AudioEvent::StartSpin);
                sleep(0.01).await;
                // let Ok((new_results, new_reward)) = get_wheel_results(pcnl_count).await else {
                //     info!("server_wheel_resutls spin error");
                //     continue;
                // };
                let keypair = if let Some(keypair) = wallet.current_keypair.peek().as_ref() {
                    keypair.insecure_clone()
                } else {
                    info!("PCNL FAIL: NO KEYPAIR!!!");
                    effects_running.set(false);
                    send_audio_event(AudioEvent::StopAudio);
                    pcnl_state.set(Some(state_init));
                    continue;
                };

                let res = get_spin_result_from_solana(keypair, *wallet.current_bet_exp.peek()).await;

                let (new_results, new_reward, log_messages) = match res {
                    Ok(((new_results, new_reward), log_messages))  => (new_results, new_reward, log_messages),
                    Err(e) => {
                        info!("PCNL FAIL : {:?}!!!", e);
                        effects_running.set(false);
                        send_audio_event(AudioEvent::StopAudio);
                        pcnl_state.set(Some(state_init));
                        continue;
                    }
                };
                wallet.do_refresh_values.call(());
                assert!(new_results.len() == state.wheels.len());
                send_audio_event(AudioEvent::HaveResults);
                if let Some(x) = pcnl_state.write().as_mut() {
                    x.last_messages = log_messages;
                }

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
                            pcnl_count,
                        });
                    }));
                }

                // wait until all pcnl is ready, then send audio stop events
                spawn(async move {
                    while !*wheels_ready.peek() {
                        sleep(0.15).await;
                    }
                    sleep(0.15).await;
                    // send_audio_event(AudioEvent::WheelsFinished);
                    if let Some(x) = pcnl_state.write().as_mut() {
                        x.last_win = if new_reward > 0 {
                            Some(new_reward)
                        } else {
                            None
                        };
                    }
                    sleep(0.15).await;
                    if new_reward > 0 {
                        for i in 0..new_reward.clamp(0, 77) {
                            send_audio_event(AudioEvent::Win { win_id: i });
                            sleep((0.40 - 0.05 * i as f64).clamp(0.15, 0.4)).await;
                        }
                    }

                    sleep(0.15).await;
                    send_audio_event(AudioEvent::WheelsFinished);
                    sleep(0.15).await;
                    send_audio_event(AudioEvent::StopAudio);
                    sleep(0.15).await;
                    effects_running.set(false);
                    if *enable_autoplay.peek() {
                        do_auto_respin.set(true);
                    }
                });
            }
        }
    });
    let tx = spin_courutine.tx();

    let tx2 = tx.clone();
    use_effect(move || {
        if *do_auto_respin.read() {
            let _ = tx2.unbounded_send(());
        }
    });

    rsx! {
        div {
            id: "spin-pcnl-btn",
            style: "height:80pt; width: 80pt;",

            {
                if *wheels_ready.read() && !*effects_running.read() && *have_money.read() {
                    rsx! {
                        button {
                            style: "width: 100%; height: 100%;",
                            onclick: move |_ev| {
                                let tx = tx.clone();
                                async move {
                                    let _ = tx.unbounded_send(());
                                }
                            },
                            h1 { "Spin" }
                        }
                    }
                } else {
                    if !*have_money.read() {
                        rsx! {
                            h1 {
                                "No credit."
                                br{}
                                a {
                                    href: "/wallet",
                                    "Open Wallet!"
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
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
    new_results: Vec<rules::Fruit>,
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
        let duration_prev = v[pcnl_id - 1].first_wait + v[pcnl_id - 1].second_wait;
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
    let mut state = use_signal(|| None);
    use_effect(move || {
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
                    pic_name: fruct.to_link_str(),
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
