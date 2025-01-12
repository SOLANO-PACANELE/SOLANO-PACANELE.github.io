use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use pacanele2_client::Account;
use pacanele2_client::FromStr;
use pacanele2_client::Keypair;
use pacanele2_client::Pubkey;
use pacanele2_client::Signer;

#[derive(Clone, Debug, Copy)]
pub struct WalletSignals {
    pub current_wallet: Signal<Option<Pubkey>>,
    pub all_wallets: Signal<Vec<SerializedKeypair>>,
    pub all_wallets_pk: Signal<Vec<Pubkey>>,
    pub current_keypair: Signal<Option<Keypair>>,
    pub current_sol: Signal<f64>,
    pub wallet_balance: Resource<HashMap<Pubkey, Option<Account>>>,
    pub set_current_wallet: Callback<Option<Pubkey>>,
    pub do_refresh_values: Callback<()>,
    pub current_credit: Signal<i64>,
    pub bet_exp_interval: Signal<Option<(u8, u8)>>,
    pub current_bet_exp: Signal<u8>,
    pub set_bet_exp: Callback<u8>,
}

pub fn init_make_wallet_selector() {
    info!("init_make_wallet_selector");

    let mut current_wallet: Signal<Option<Pubkey>> = dioxus_sdk::storage::use_synced_storage::<
        dioxus_sdk::storage::LocalStorage,
        Option<Pubkey>,
    >(
        "current_wallet_pubkey".to_string(), || None
    );

    let all_wallets = dioxus_sdk::storage::use_synced_storage::<
        dioxus_sdk::storage::LocalStorage,
        Vec<SerializedKeypair>,
    >("wallet_keypairs".to_string(), || vec![]);

    let mut current_keypair = use_signal(|| None);
    use_effect(move || {
        if let Some(w_pk) = current_wallet.read().as_ref() {
            info!("current_keypair() : have account = {}", w_pk);
            let w0 = all_wallets;
            let v = w0
                .peek()
                .iter()
                .cloned()
                .map(|k| k.keypair())
                .filter(|k| k.pubkey() == *w_pk)
                .collect::<Vec<_>>();
            if v.is_empty() {
                return;
            }
            info!("current_keypair() : have keypairs: {}", v.len());
            let sender = v.into_iter().next().unwrap();
            current_keypair.set(Some(sender));
        }
    });

    let mut wallet_balance: Resource<HashMap<Pubkey, Option<Account>>> = use_resource(move || {
        let all_wallets = all_wallets.read().clone();

        async move {
            let client = pacanele2_client::get_client().await;
            let mut hash = HashMap::<Pubkey, Option<Account>>::new();
            for w in all_wallets.iter() {
                let w = w.keypair().pubkey();
                hash.insert(w, client.get_account(&w).await.ok());
            }
            hash
        }
    });

    let mut current_sol = use_signal(move || 0.0);
    use_effect(move || {
        let w2 = wallet_balance.read();

        if let Some(key) = *current_wallet.read() {
            if let Some(hash) = w2.as_ref() {
                if let Some(Some(acc)) = hash.get(&key) {
                    let val = acc.lamports as f64 / 1000000000.0;
                    current_sol.set(val);
                } else {
                    current_sol.set(0.0);
                }
            } else {
                info!("no current hash!");
                current_sol.set(0.0);
            }
        } else {
            info!("no current wallet!")
        }
    });

    let bet_interval_res = use_resource(move || {
        // re-run bet interval when account changes
        let key = current_wallet.read().as_ref().cloned();
        // re-run bet interval when current solana changes
        let _sol = *current_sol.read();
        async move {
            if _sol > 0.0 {
                if let Some(key) = key {
                    let client = pacanele2_client::get_client().await;
                    pacanele2_client::pcnl_possible_bet_interval(&client, &key).await
                } else {
                    Err("no current account".to_string())
                }
            } else {
                Err("no solana in account".to_string())
            }
        }
    });
    let mut bet_exp_interval = use_signal(|| None);
    let mut current_bet_exp = use_signal(|| 0);
    use_effect(move || {
        let mut c_e = current_bet_exp.write();
        if let Some(Ok(x)) = bet_interval_res.read().as_ref() {
            let (min, max) = *x;
            bet_exp_interval.set(Some(*x));
            *c_e = (*c_e).clamp(min, max);
        } else {
            bet_exp_interval.set(None);
            *c_e = 0;
        }
    });

    let mut all_wallets_pk = use_signal(move || vec![]);
    use_effect(move || {
        let w = all_wallets
            .read()
            .iter()
            .map(|k| k.keypair().pubkey())
            .collect::<Vec<_>>();
        all_wallets_pk.set(w);
    });

    let do_refresh_values = Callback::new(move |_| {
        wallet_balance.restart();
    });

    let set_current_wallet = Callback::new(move |x_| {
        current_wallet.set(x_);
        do_refresh_values.call(());
    });

    let mut current_credit = use_signal(move || 0);
    use_effect(move || {
        let sol = *current_sol.read();
        let bet_exp = *current_bet_exp.read();
        if bet_exp > 10 && bet_exp < 63 && sol > 0.0 {
            let bet_exp_lamp = 1_u64 << (bet_exp as u64);
            let credit_float  = sol / (bet_exp_lamp as f64 / 1000000000.0);
            let credit = credit_float as i64;
            current_credit.set(credit);
        } else {
            current_credit.set(0);
        }
    });
    let set_bet_exp = Callback::new(move |_new: u8| {
        let x = (*bet_exp_interval.read()).unwrap_or_default();
        let _new = _new.clamp(x.0, x.1);
        if _new > 10 && _new < 63 {
            *current_bet_exp.write() = _new;
        }
    });

    use_context_provider(move || WalletSignals {
        current_credit,
        current_wallet,
        all_wallets,
        current_keypair,
        current_sol,
        all_wallets_pk,
        wallet_balance,
        set_current_wallet,
        do_refresh_values,bet_exp_interval,current_bet_exp, set_bet_exp
    });
}

pub fn wallet_signals() -> WalletSignals {
    use_context::<WalletSignals>()
}

#[component]
pub fn BetAmountControl() -> Element {
    let w = wallet_signals();
    let bet_int = (*w.bet_exp_interval.read()).unwrap_or_default();
    let bet_exp = (*w.current_bet_exp.read()).clamp(bet_int.0, bet_int.1);

    let bet = (1_u64 << (bet_exp as u64)) as f64 / 1000000000.0;

    rsx! {
        if bet_exp > 0 {
            h1 {
                "Bet Amount: {bet} SOL",
            }
            button {
                disabled: bet_exp <= bet_int.0,
                onclick: move |_| {
                    if bet_exp > 10 {
                        w.set_bet_exp.call(bet_exp-1);
                    }
                },
                h1 {"Bet -"},
            },
            button {
                disabled: bet_exp >= bet_int.1,
                onclick: move |_| {
                    if bet_exp < 63 {
                        w.set_bet_exp.call(bet_exp+1);
                    }
                },
                h1 {"Bet +"},
            },
            button {
                disabled: bet_exp >= bet_int.1,
                onclick: move |_| {
                    if bet_exp < 63 {
                        w.set_bet_exp.call(bet_int.1);
                    }
                },
                h1 {"Bet MAX"},
            },
        }
    }
}

#[component]
pub fn CurrentWalletDropdown() -> Element {
    let w: WalletSignals = wallet_signals();

    let ev_to_data = move |_ev: Event<FormData>| -> String {
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
                    w.set_current_wallet.call(Pubkey::from_str(&val).ok());
                },

                for pubkey in w.all_wallets_pk.read().iter() {
                    option {
                        value: "{pubkey}",
                        " {pubkey} = ",
                        {
                            format!("{:?}", if let Some(hash) = w.wallet_balance.read().as_ref() {
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
                "{w.current_sol:?} SOL"
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
    pub fn keypair(&self) -> Keypair {
        Keypair::from_bytes(&self.0).unwrap()
    }
}

#[component]
pub fn WalletDashboard() -> Element {
    rsx! {
        PlayerAccountList {accounts:wallet_signals().all_wallets}
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

    let mut import_private_key_json = use_signal(move || "".to_string());
    let parsed_keypair = use_memo(move || {
        let Ok(b) = serde_json::from_str::<Vec<u8>>(import_private_key_json.read().as_ref()) else {return None;};
        Some(b)
    });

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
                    "+ add new account"
                }
                br {}
                input {
                    value: "{import_private_key_json}",
                    oninput: move |event| import_private_key_json.set(event.value())
                }
                button {
                    onclick: move |_| {
                        info!("import acount!");
                        if let Some(json_b) = parsed_keypair.read().as_ref()  {
                            if let Ok(key) = pacanele2_client::Keypair::from_bytes(&json_b) {
                                accounts.write().push(key.into());
                            }
                        }
                    },
                    "+ import private key json"
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
    let wallet = wallet_signals();

    if Some(account) == wallet.current_wallet.read().clone() {
        rsx! {
            h1 {
                "Is current Wallet"
            }
        }
    } else {
        rsx! {
            button {
                onclick: move |_| {
                    wallet.set_current_wallet.call(Some(account));
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
