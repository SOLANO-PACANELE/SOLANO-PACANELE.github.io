use core::f64;

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

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
        document::Style {
            {make_animation_string()}
        }
        Router::<Route> {}
    }
}

fn make_animation_string() -> String {
    let mut css = "".to_string();
    css.push_str("@keyframes spin { ");
    for x in 0..=100 {
        let deg = format!("{}deg", 360 * x / 100);
        let rad = 2. * f64::consts::PI / 100.0;
        let y = rad.sin() ;
        let z = rad.cos() + 2.0 ;
        let line_rule = format!(" transform: perspective(1cqmin) translate3d(0, {y}cqmin, {z}cqmin) rotate3d(1, 0, 0, {deg} ) ; ");
        let line_css = format!("{x}% {{ {line_rule} }}");
        css.push_str(&line_css);
    }
    css.push_str("}");
    css
}


#[component]
fn Pacanele() -> Element {
    rsx! {

        div {
            id: "top-box"
        }
        div {
            id: "left-box"
        }
        div {
            id: "bottom-box"
        }
        div {
            id: "right-box"
        }

        div {
            id: "pacanele",

            div {
                id: "x777",

                div {
                    id: "slot1",
                    SlotImage { pic_name: "orange".to_string() }
                }
                div {
                    id: "slot2",
                    SlotImage { pic_name: "orange".to_string() }
                }
                div {
                    id: "slot3",
                    SlotImage { pic_name: "orange".to_string() }
                }
            }

        }
    }
}

#[component]
fn SlotImage(pic_name: String) -> Element {
    rsx! {
        img {
            class: "fruit-image",
            src: format!("/assets/img2/fruit/{pic_name}.png")
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
