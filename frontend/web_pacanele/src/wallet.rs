use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use pacanele2_client::FromStr;
use pacanele2_client::Keypair;
use pacanele2_client::Pubkey;
use pacanele2_client::Signer;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SerializedKeypair(Vec<u8>);

impl From<SerializedKeypair> for Keypair {
    fn from(value: SerializedKeypair) -> Self {
        Keypair::from_bytes(&value.0).unwrap()
    }
}

impl From<Keypair> for SerializedKeypair {
    fn from(value: Keypair) -> Self {
        SerializedKeypair(value.to_bytes().to_vec())
    }
}

impl SerializedKeypair {
    fn keypair(&self) -> Keypair {
        Keypair::from_bytes(&self.0).unwrap()
    }
}

#[component]
pub fn WalletDashboard() -> Element {
    let accounts = dioxus_sdk::storage::use_synced_storage::<
        dioxus_sdk::storage::LocalStorage,
        Vec<SerializedKeypair>,
    >("wallet_keypairs".to_string(), || vec![]);

    rsx! {
        PlayerAccountList {accounts}
    }
}

#[component]
fn PlayerAccountList(accounts: Signal<Vec<SerializedKeypair>>) -> Element {
    let bank_address = pacanele2_client::get_bank_address().0;
    let delete_me = move |account| {
        accounts.write().retain(|k| k.keypair().pubkey() != account);
    };

    let send_money = move |(sender, target, amount)| {
        let accounts = accounts.peek();
        let v = accounts
            .iter()
            .filter(|k| k.keypair().pubkey() == sender)
            .collect::<Vec<_>>();
        let sender = v.first();
        if let Some(sender) = sender {
            let sender = sender.keypair();
            let sender_pub = sender.pubkey();
            spawn(async move {
                let client = pacanele2_client::get_client().await;
                let result = pacanele2_client::send_money(&client, sender, target, amount).await;
                info!(
                    "SEND MONEY: FROM={} TO={} AMOUNT={} \n TX={:#?}",
                    sender_pub, target, amount, result
                );
            });
        }
    };

    rsx! {
        div {
            style: "border: 1px solid black;",
            h1 {                "Bank"             }
            PlayerAccountDisplay {account:bank_address, on_forget:delete_me, send_money}

            h1 {
                "Player Wallets"
            }
            h1 {
                button {
                    onclick: move |_| async move {
                        info!("add account!");
                        let key = pacanele2_client::create_new_keypair();
                        accounts.write().push(key.into());
                    },
                    "+ add account"
                }
            }

            ul {
                for account in accounts.read().iter().cloned() {
                    li {
                        key: account.keypair().pubkey().to_string(),
                        PlayerAccountDisplay {account: account.keypair().pubkey(), on_forget:delete_me, send_money:send_money}

                    }
                }
            }
        }
    }
}

#[component]
fn PlayerAccountDisplay(
    account: Pubkey,
    on_forget: Callback<Pubkey>,
    send_money: Callback<(Pubkey, Pubkey, u64)>,
) -> Element {
    let mut account_info = use_resource(move || async move {
        let cl = pacanele2_client::get_client().await;
        cl.get_account(&account).await
    });
    let solana_value = use_memo(move || {
        if let Some(Ok(i)) = account_info.read().as_ref() {
            i.lamports as f64 / 1000000000.0
        } else {
            0.0
        }
    });

    rsx! {
        div {
            style: "display:flex;",
            pre {
                style: "border: solid black 1px; width:max-content; padding: 10pt; margin: 10pt;",

                "Address (pubkey): {account}",
                br {}
                "{account_info.read().as_ref():#?}",
            }
            div {
                style: "border: 1px solid red; width: max-content; padding: 10pt; margin: 10pt; display:grid;",

                h1 {
                    style:"color:blue;",
                    "SOL: {solana_value}"
                }
                button {
                    onclick: move |_| {
                        account_info.restart();
                    },
                    "refresh"
                }

                button {
                    onclick: move |_| async move {
                        info!("airdrop {}", account);
                        let client = pacanele2_client::get_client().await;
                        pacanele2_client::request_airdrop(&client, &account, 5).await;
                        info!("airdrop ok");
                        account_info.restart();

                    },
                    "airdrop +5"
                }
                PlayerAccountSendMoneyButton {account, refresh_account: move || {
                    account_info.restart();
                }, send_money}
                button {
                    onclick: move |_| on_forget.call(account),
                    "forget"
                }
            }
        }
    }
}

#[component]
fn PlayerAccountSendMoneyButton(
    account: Pubkey,
    refresh_account: Callback<(), ()>,
    send_money: Callback<(Pubkey, Pubkey, u64)>,
) -> Element {
    let mut target = use_signal(|| "".to_string());
    let mut amount: Signal<String> = use_signal(|| "0".to_string());

    rsx! {
        div {
            input {
                value: "{target}",
                oninput: move |event| target.set(event.value())
            }
            input {
                r#type:"number",
                value: "{amount}",
                oninput: move |event| amount.set(event.value())
            }
            button {
                onclick: move |_| {
                    let Ok(am) = amount.peek().parse() else {return;};

                    if let Ok(target_pubkey) = Pubkey::from_str(target.peek().as_ref()) {
                        send_money.call((account, target_pubkey, am));
                        refresh_account.call(());
                    }
                },
                "send {amount}"
            }
        }
    }
}
