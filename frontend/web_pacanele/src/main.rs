use core::f64;

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};
use svg_attributes::fr;


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
            {make_animation_string(get_all_fruits().len() as i32)}
        }
        Router::<Route> {}
    }
}

fn make_animation_string(pic_count: i32) -> String {
    let mut css = "".to_string();
    css.push_str("@keyframes spin { ");
    const num_keyframes: i32 = 25;
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

fn make_transform_string(rad: f64, pic_count: i32) -> String {
    let size_coef = 2.9;
    let y = rad.sin() *size_coef * 100.0;
    let z = (rad.cos() - 1.5)*size_coef * 100.0;
    let z_index =  ((rad.cos() - 2.0) * 100.0).round() as i32;
    let scale = 2.0 * f64::consts::PI / pic_count as f64 * 1.05 * size_coef;
    // info!("transform c={pic_count}   scale={scale}");
    format!(" transform:
    perspective(555cqmin)
     translate3d(0cqmin, {y}cqmin, {z}cqmin) 
     rotate3d(1, 0, 0, {rad}rad) scale3d({scale},{scale},{scale});

     z-index: {z_index};
    ")
}

fn get_all_fruits() -> Vec<String> {
   include_str!("./fruits.txt").split_whitespace().map(|s| s.trim().split(".").next().unwrap().to_string()).collect()

}


fn shuffle_fruit(v: & Vec<String>) -> Vec<String> {
    let mut v = v.clone();
    let mut rng = rand::thread_rng();
    use rand::prelude::SliceRandom;
    v.shuffle(&mut rng);
    v
}

#[component]
fn Pacanele() -> Element {
    let fruit_list = get_all_fruits();
    let fruit_count = fruit_list.len() as u64;
    let rand_pos = move || {
        let mut rng = rand::thread_rng();
        use rand::Rng;
        rng.gen::<u64>() % fruit_count
    };
    let mut positions = use_signal(|| (0,0,0));
    let mut fetch_new_positions = move || {
        let old_pos = *positions.peek();
        let new_pos = (rand_pos(), rand_pos(), rand_pos());
        positions.set(new_pos);
        (old_pos, new_pos)
    };

    let mut spin_sequence = use_signal(|| ((0,0,0), (0,0,0)));

    let mut do_spin = move || {
        spin_sequence.set(fetch_new_positions());
    };

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
            id: "right-box",
            button {
                onclick: move |_ev| {
                    do_spin();
                },
                h1 {
                    "Spin"
                }
            }
        }

        div {
            id: "pacanele",

            div {
                id: "x777",

                SlotWheel { div_id: "slot1".to_string(), fruit_list: fruit_list.clone() , spin_period: 3.0, spin_from: spin_sequence.read().0.0, spin_to: spin_sequence.read().1.0 }
                SlotWheel { div_id: "slot2".to_string(), fruit_list:  fruit_list.clone(), spin_period: 4.0, spin_from: spin_sequence.read().0.0, spin_to: spin_sequence.read().1.0 }
                SlotWheel { div_id: "slot3".to_string(), fruit_list: fruit_list.clone(), spin_period: 5.0, spin_from: spin_sequence.read().0.0, spin_to: spin_sequence.read().1.0 } 
            }
        }

    }
}


#[component]
fn SlotWheel(fruit_list: Vec<String>, div_id: String, spin_period: f64, spin_from: u64, spin_to: u64) -> Element {
    rsx! {

    div {
        id: div_id,
        class: "slot-box",

        div {
            class: "slot-display",
            div {
                class:"pavaravan"
            }
    
            for (i, fruct) in fruit_list.iter().enumerate() {
                SlotImage { pic_name: fruct.to_string(), pic_pos: i as i32, pic_count: fruit_list.len() as i32 , spin_period}
            }
        }

    }
}

}

#[component]
fn SlotImage(pic_name: String, pic_pos: i32, pic_count: i32, spin_period: f64) -> Element {
    let spin_count = 1.0 + pic_pos as f64 / pic_count as f64 ;
    let delay = spin_period * pic_pos as f64 / pic_count as f64 ;

    let rad = 2. * f64::consts::PI * pic_pos as f64 / pic_count as f64;
    let final_transform = make_transform_string(rad, pic_count);
    rsx! {
        img {
            class: "fruit-image",
            style: format!("
                animation:  {spin_period}s linear {spin_count} -{delay}s spin ;
                {final_transform}
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
