
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use rules::Fruit;
use web_pacanele::{
    audio::make_audio_loop_coroutine,
    gen_css::make_animation_string,
};

use web_pacanele::client::SolanaDemo;
use web_pacanele::pacanele::Pacanele;
use web_pacanele::wallet::{make_wallet_selector, WalletDashboard};

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
    make_wallet_selector();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { {make_animation_string("spin_1", Fruit::all().len() as u32)} }
        document::Style { {make_animation_string("spin_2", Fruit::all().len() as u32)} }
        Router::<Route> {}
    }
}
