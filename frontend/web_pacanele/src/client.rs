use dioxus::prelude::*;
use dioxus_logger::tracing::*;
use pacanele2_client::{Keypair, Signer};
use rules::Fruit;

use crate::wallet::{wallet_signals, CurrentWalletDropdown};

#[component]
pub fn SolanaDemo() -> Element {
    info!("SolanaDemo");
    let wallet = wallet_signals();

    let mut output = use_signal(move || "".to_string());

    rsx! {
        div {
            h2 {
                "Solana Demo"
            }
            CurrentWalletDropdown {}

            button {
                onclick: move |_| {
                    async move {
                        if let Some(k) = wallet.current_keypair.peek().as_ref() {
                            let xr = get_spin_result_from_solana(k.insecure_clone()).await;
                            wallet.do_refresh_values.call(());
                            let xr = format!("{:#?}", xr);
                            output.set(xr);
                        } else {
                            output.set("no current keypair".to_string());
                        }

                    }
                },
                h3 {"Send"}
            }
            h4 {
                pre {
                    "{output}"
                }
            }
        }
    }
}

pub async fn get_spin_result_from_solana(
    sender: Keypair,
) -> Result<((Vec<Fruit>, u16), Vec<String>), String> {
    // return Ok(((vec![Fruit::seven;3], 12345), vec!["fake".to_string()]));

    info!("get_spin_result_from_solana()");
    let client = pacanele2_client::get_client().await;
    use rules::Fruit;
    let x = pacanele2_client::spin_pcnl(&client, sender).await?;
    info!(
        "get_spin_result_from_solana() : final transaction status = {:?}",
        x.status
    );
    let b = pacanele2_client::base64_decode_return(&x)?;
    let xr = bincode::deserialize::<(Vec<Fruit>, u16)>(&b).map_err(|e| format!("{:?}", e))?;
    info!("get_spin_result_from_solana() : xr = {:?}", xr);
    Ok((xr, x.log_messages.clone().unwrap()))
}
