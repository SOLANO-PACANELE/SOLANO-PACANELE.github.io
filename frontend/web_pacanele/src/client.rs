use dioxus::prelude::*;
use dioxus_elements::h1;
use dioxus_logger::tracing::*;
use rules::Fruit;

#[component]
pub fn SolanaDemo() -> Element {
    info!("SolanaDemo");
    let mut s = use_signal(move || "".to_string());

    rsx! {
        div {
            h2 {
                "Solana Demo"
            }
            button {
                onclick: move |_| {
                    async move {
                        let x = pacanele2_client::demo().await.unwrap();
                        let x = pacanele2_client::base64_decode_return(&x);
                        let xr = bincode::deserialize::<(Vec<Fruit>, u16)>(&x);
                        let xr = format!("{:?}", xr);

                        s.set(xr);
                    }
                },
                h3 {"Send"}
            }
            h4 {
                pre {
                    "{s}"
                }
            }
        }
    }
}

pub async fn get_spin_result_from_solana() -> ((Vec<Fruit>, u16), Vec<String>) {
    use rules::Fruit;
    let x = pacanele2_client::demo().await.unwrap();
    info!("{:#?}", x);
    let b = pacanele2_client::base64_decode_return(&x);
    let xr = bincode::deserialize::<(Vec<Fruit>, u16)>(&b).unwrap();
    (xr, x.log_messages.clone().unwrap())
}