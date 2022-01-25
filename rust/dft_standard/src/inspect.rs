extern crate dft_types;
extern crate dft_utils;

use candid::Nat;
use crate::token::TokenStandard;
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::inspect_message;

use crate::state::TOKEN;

static QUERY_METHODS: [&str; 17] = [
    "allowance",
    "allowancesOf",
    "balanceOf",
    "decimals",
    "desc",
    "fee",
    "logo",
    "meta",
    "name",
    "owner",
    "symbol",
    "tokenInfo",
    "totalSupply",
    "lastTransactions",
    "transactionById",
    "transactionByIndex",
    "__get_candid_interface_tmp_hack"
];

static OWNER_METHODS: [&str; 6] = [
    "setDesc", "setFee", "setFeeTo", "setFeeTo", "setLogo", "setOwner",
];
static HOLDER_METHODS: [&str; 3] = ["approve", "transfer", "burn"];

//static SPENDER_METHODS: [&str; 3] = ["transferFrom", "burnFrom"];
#[inspect_message]
fn inspect_message() {
    let method = api::call::method_name();
    let caller = api::caller();

    match &method[..] {
        m if QUERY_METHODS.contains(&m) => api::call::accept_message(),
        m if HOLDER_METHODS.contains(&m) => {
            let holder = match m {
                "approve" => {
                    let (sub_account, _, _) =
                        api::call::arg_data::<(Option<Subaccount>, String, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                "transfer" => {
                    let (sub_account, _, _) =
                        api::call::arg_data::<(Option<Subaccount>, String, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                "burn" => {
                    let (sub_account, _) = api::call::arg_data::<(Option<Subaccount>, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                // nover match this case
                _ => TokenHolder::new(caller, None),
            };

            // check caller's balance
            let balance = TOKEN.with(|token| token.borrow().balance_of(&holder));
            if balance > Nat::from(0) {
                api::call::accept_message();
            } else {
                ic_cdk::println!("inspect: caller's balance is 0; reject");
                api::call::reject_message();
            }
        }
        m if OWNER_METHODS.contains(&m) => {
            // check if caller is owner
            let owner = TOKEN.with(|token| token.borrow().owner());
            if caller == owner {
                api::call::accept_message();
            } else {
                ic_cdk::println!("inspect: caller is not owner; reject");
                api::call::reject_message();
            }
        }
        _ => {
            ic_cdk::println!("inspect: method not checked; accept");
        }
    }
}
