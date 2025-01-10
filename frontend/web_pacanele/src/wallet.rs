
use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use pacanele2_client::Account;
use pacanele2_client::FromStr;
use pacanele2_client::Keypair;
use pacanele2_client::Pubkey;
use pacanele2_client::Signer;


#[derive(Clone, Debug, Copy)]
struct WalletSelector{
    current_wallet: Signal<Option<Pubkey>>,
    all_wallets:  Signal<Vec<SerializedKeypair>>,
}


pub fn make_wallet_selector()  {
    let current_wallet: Signal<Option<Pubkey>> = dioxus_sdk::storage::use_synced_storage::<
    dioxus_sdk::storage::LocalStorage,
    Option<Pubkey>,
>("current_wallet_pubkey".to_string(), || None);

    let all_wallets = dioxus_sdk::storage::use_synced_storage::<
    dioxus_sdk::storage::LocalStorage,
    Vec<SerializedKeypair>,
>("wallet_keypairs".to_string(), || vec![]);
    use_context_provider(move || WalletSelector{current_wallet, all_wallets});
}

pub fn current_wallet() -> Signal<Option<Pubkey>> {
    let s = use_context::<WalletSelector>();
    s.current_wallet
}

pub fn all_wallets() -> Signal<Vec<SerializedKeypair>> {
    use_context::<WalletSelector>().all_wallets
}

#[component]
pub fn CurrentWalletDropdown() -> Element {
    let all_wallet_serial = all_wallets();
    let mut all_wallets = use_signal(move || vec![]);
    use_effect(move || {
        info!("all_wallets()");
        let w = all_wallet_serial
            .read().iter()
            .map(|k| k.keypair().pubkey())
            .collect::<Vec<_>>();
        all_wallets.set(w);
    });

    let mut current_wallet = current_wallet();

    let wallet_balance = use_resource(move || async move {
        info!("wallet_balance()");
        let client = pacanele2_client::get_client().await;
        let mut hash = HashMap::<Pubkey, Option<Account>>::new();
        for w in  all_wallets.read().iter() 
        {
            hash.insert(*w, client.get_account(w).await.ok());
        }
        hash
    });

    let current_balance = use_memo(move || {
        info!("current_balance()");
        if let Some(key) = *current_wallet.read() {
            if let Some(hash) = wallet_balance.read().as_ref() {
                if let Some(Some(acc)) = hash.get(&key) {
                    return acc.lamports as f64 / 1000000000.0;
                }
            }
        }
        return 0.0;
    });

    let ev_to_data = move |_ev: Event<FormData>| -> String {
        // info!("VAL: {_ev:?}");
        let data = _ev.data().clone();
        let value = data.value();

        value
    };


    rsx! {
        div {
            style: "display:flex;  align-items: center;",
            label {
                r#for: "current_account",
                h1 {
                    "Current Account"
                }
            },
            select {
                name: "current_account",
                id: "current_account",
                oninput: move |_ev| {
                    let val = ev_to_data(_ev);
                    current_wallet.set(Pubkey::from_str(&val).ok());
                },
    
                for pubkey in all_wallets.read().iter() {
                    option {
                        value: "{pubkey}",
                        " {pubkey} = ",
                        {
                            format!("{:?}", if let Some(hash) = wallet_balance.read().as_ref() {
                                if let Some(Some(acc)) = hash.get(&pubkey) {
                                    Some( acc.lamports as f64 / 1000000000.0)
                                } else {
                                    None
                                }
                            } else {
                                None
                            })
                        } , 
                        "SOL"
                    }
                }
            }
            h1 {
                "{current_balance:?} SOL"
            }

        }

        // <label for="cars">Choose a car:</label>

        // <select name="cars" id="cars">
        //   <option value="volvo">Volvo</option>
        //   <option value="saab">Saab</option>
        //   <option value="mercedes">Mercedes</option>
        //   <option value="audi">Audi</option>
        // </select> 
    }
}


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


    rsx! {
        PlayerAccountList {accounts:all_wallets()}
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
            div {
                style: "border: solid black 1px; width:max-content; padding: 10pt; margin: 10pt;",

                DisplayCurrentWallet{account}
            }

        }
    }
}

#[component]
fn DisplayCurrentWallet(account: Pubkey) -> Element {
    let mut wallet = current_wallet();

    if Some(account) == wallet.read().clone() {
        rsx! {
            h1 {
                "Is current Wallet"
            }
        }
    } else {
        rsx! {
            button {
                onclick: move |_| {
                    wallet.set(Some(account));
                },

                "Set current Wallet",
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
