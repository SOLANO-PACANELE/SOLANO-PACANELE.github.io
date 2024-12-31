use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Pacanele {},

}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
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
