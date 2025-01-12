use dioxus::prelude::*;
use dioxus_logger::tracing::*;
use pacanele2_client::{Keypair, Signer};
use rules::Fruit;

use crate::wallet::{wallet_signals, BetAmountControl, CurrentWalletDropdown};

#[component]
pub fn SolanaDemo() -> Element {
    let wallet = wallet_signals();

    let mut output = use_signal(move || "".to_string());

    rsx! {
        div {
            h2 {
                "Solana Demo"
            }
            CurrentWalletDropdown {}
            BetAmountControl {}

            button {
                onclick: move |_| {
                    async move {
                        if let Some(k) = wallet.current_keypair.peek().as_ref() {
                            let xr = get_spin_result_from_solana(k.insecure_clone(), *wallet.current_bet_exp.peek()).await;
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
    bet_amount_exp: u8,
) -> Result<((Vec<Fruit>, u16), Vec<String>), String> {
    // return Ok(((vec![Fruit::seven;3], 12345), vec!["fake".to_string()]));

    let client = pacanele2_client::get_client().await;
    use rules::Fruit;
    let x = pacanele2_client::spin_pcnl(&client, sender, bet_amount_exp).await?;

    let b = pacanele2_client::base64_decode_return(&x)?;
    let xr = bincode::deserialize::<(Vec<Fruit>, u16)>(&b).map_err(|e| format!("{:?}", e))?;
    Ok((xr, x.log_messages.clone().unwrap()))
}
