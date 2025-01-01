use core::f64;

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

const spin_period: f64 = 50.5_f64;

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
    const n: i32 = 100;
    for x in 0..=n {
        let deg = format!("{}deg", -360.0 * x as f64 / n as f64);
        let rad = 2. * f64::consts::PI * x as f64 / n as f64;
        let y = rad.sin() *100.0;
        let z = (rad.cos() - 1.0)*100.0 ;
        let my = -y;
        let mz = -z;
        let line_rule = format!(" transform:
             perspective(100cqmin)
              translate3d(0, {y}cqmin, {z}cqmin) 
              rotate3d(1, 0, 0, {deg} ) ;

              z-index: {z};
             ");
        let line_css = format!("{x}% {{ {line_rule} }}");
        css.push_str(&line_css);
    }
    css.push_str("}");
    css
}

const LEGUME : [&'static str; 4] = [
    "orange",
    "seven", 
    "strawberry",
    "watermelon",
];

#[component]
fn Pacanele() -> Element {
    let  mut fruit_list = vec![];
    for _x in 0..3 {
        for j in LEGUME {
            fruit_list.push(j.to_string());
        }
    }
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
                SlotWheel { div_id: "slot1".to_string(), fruit_list: fruit_list.clone() }
                SlotWheel { div_id: "slot2".to_string(), fruit_list: fruit_list.clone() }
                SlotWheel { div_id: "slot3".to_string(), fruit_list: fruit_list.clone() } 

            }
        }
    }
}


#[component]
fn SlotWheel(fruit_list: Vec<String>, div_id: String) -> Element {
    rsx! {

    div {
        id: div_id,
        for (i, fruct) in fruit_list.iter().enumerate() {
            SlotImage { pic_name: fruct.to_string(), pic_pos: i as i32, pic_count: fruit_list.len() as i32 }
        }
    }
}

}

#[component]
fn SlotImage(pic_name: String, pic_pos: i32, pic_count: i32) -> Element {
 
    let delay = spin_period * pic_pos as f64 / pic_count as f64 ;
    rsx! {
        img {
            class: "fruit-image",
            style: format!("
                animation:spin  {spin_period}s linear infinite;
                animation-delay: -{delay}s;
            "),
            src: format!("/assets/img2/fruit/{pic_name}.png")
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
