use core::f64;
use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use rules::{rule_set::RuleSet, Fruit};
use web_pacanele::{
    audio::{make_audio_loop_coroutine, send_audio_event, AudioEvent},
    gen_css::{make_animation_string, make_transform_string},
    random::{get_wheel_results, get_wheel_shuffle},
    state::{PcnlState, PcnlWheelState, ShuffleState, WheelShuffleState, WheelStage},
    time::{get_current_ts, sleep},
};

use web_pacanele::client::SolanaDemo;
use web_pacanele::pacanele::Pacanele;
use web_pacanele::wallet::WalletDashboard;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Pacanele {},
    #[route("/demo")]
    SolanaDemo {},
    #[route("/wallet")]
    WalletDashboard {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    make_audio_loop_coroutine();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { {make_animation_string("spin_1", Fruit::all().len() as u32)} }
        document::Style { {make_animation_string("spin_2", Fruit::all().len() as u32)} }
        Router::<Route> {}
    }
}
