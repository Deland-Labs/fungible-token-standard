use candid::candid_method;
use dft_standard::token::TokenStandard;
use dft_standard::{auto_scaling_storage::exec_auto_scaling_strategy, state::TOKEN};
use dft_types::*;
use dft_utils::*;
use ic_cdk::{
    api,
    export::{candid::Nat, Principal},
};
use ic_cdk_macros::*;
use std::string::String;

#[init]
async fn canister_init(
    sub_account: Option<Subaccount>,
    logo_: Option<Vec<u8>>,
    name_: String,
    symbol_: String,
    decimals_: u8,
    total_supply_: Nat,
    fee_: Fee,
    caller: Option<Principal>,
) {
    let real_caller = caller.unwrap_or_else(|| api::caller());
    let owner_holder = TokenHolder::new(real_caller, sub_account);
    // convert logo to Option<Vec<u8>>
    // token initialize
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.initialize(
            &real_caller,
            api::id(),
            logo_,
            name_,
            symbol_,
            decimals_,
            fee_,
            owner_holder.clone(),
        );
        let _ = token._mint(
            &real_caller,
            &owner_holder,
            total_supply_,
            None,
            api::time(),
        );
    });
}

#[query(name = "owner")]
#[candid_method(query, rename = "owner")]
fn owner() -> Principal {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.owner()
    })
}

#[query(name = "name")]
#[candid_method(query, rename = "name")]
fn get_name() -> String {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.name()
    })
}

#[query(name = "symbol")]
#[candid_method(query, rename = "symbol")]
fn get_symbol() -> String {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.symbol()
    })
}

#[query(name = "decimals")]
#[candid_method(query, rename = "decimals")]
fn get_decimals() -> u8 {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.decimals()
    })
}

#[query(name = "totalSupply")]
#[candid_method(query, rename = "totalSupply")]
fn get_total_supply() -> Nat {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.total_supply()
    })
}

#[query(name = "fee")]
#[candid_method(query, rename = "fee")]
fn get_fee_setting() -> Fee {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.fee()
    })
}

#[query(name = "meta")]
#[candid_method(query, rename = "meta")]
fn get_meta_data() -> Metadata {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.metadata()
    })
}

#[query(name = "desc")]
#[candid_method(query, rename = "desc")]
fn get_desc_info() -> Vec<(String, String)> {
    TOKEN.with(|token| {
        let token = token.borrow();
        // get token desc , return as a vector
        token
            .desc()
            .iter()
            .map(|v| (v.0.clone(), v.1.clone()))
            .collect()
    })
}

#[query(name = "logo")]
#[candid_method(query, rename = "logo")]
fn logo() -> Vec<u8> {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.logo()
    })
}

#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(holder: String) -> Nat {
    let token_holder_parse_result = holder.parse::<TokenHolder>();
    match token_holder_parse_result {
        Ok(token_holder) => TOKEN.with(|token| {
            let token = token.borrow();
            token.balance_of(&token_holder)
        }),
        _ => Nat::from(0),
    }
}

#[query(name = "nonceOf")]
#[candid_method(query, rename = "nonceOf")]
fn nonce_of(principal: Principal) -> u64 {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.nonce_of(&principal)
    })
}

#[query(name = "allowance")]
#[candid_method(query, rename = "allowance")]
fn allowance(owner: String, spender: String) -> Nat {
    let token_holder_owner_parse_result = owner.parse::<TokenHolder>();
    let token_holder_spender_parse_result = spender.parse::<TokenHolder>();

    if let Ok(token_holder_owner) = token_holder_owner_parse_result {
        if let Ok(token_holder_spender) = token_holder_spender_parse_result {
            return TOKEN.with(|token| {
                let token = token.borrow();
                token.allowance(&token_holder_owner, &token_holder_spender)
            });
        }
    }

    Nat::from(0)
}

#[update(name = "approve")]
#[candid_method(update, rename = "approve")]
async fn approve(
    owner_sub_account: Option<Subaccount>,
    spender: String,
    value: Nat,
    nonce: Option<u64>,
) -> TransactionResult {
    let caller = api::caller();
    let owner_holder = TokenHolder::new(caller.clone(), owner_sub_account);
    match spender.parse::<TokenHolder>() {
        Ok(spender_holder) => {
            match TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.approve(
                    &caller,
                    &owner_holder,
                    &spender_holder,
                    value,
                    nonce,
                    api::time(),
                )
            }) {
                Ok(tx_index) => {
                    let tx_id = encode_tx_id(api::id(), tx_index);

                    TransactionResult::Ok(TransactionResponse {
                        tx_id,
                        error: match exec_auto_scaling_strategy().await {
                            Ok(_) => None,
                            Err(e) => Some(e),
                        },
                    })
                }
                Err(e) => TransactionResult::Err(e.into()),
            }
        }
        Err(_) => TransactionResult::Err(DFTError::InvalidSpender.into()),
    }
}

#[query(name = "allowancesOf")]
#[candid_method(query, rename = "allowancesOf")]
fn allowances_of_holder(holder: String) -> Vec<(TokenHolder, Nat)> {
    match holder.parse::<TokenHolder>() {
        Ok(token_holder) => TOKEN.with(|token| {
            let token = token.borrow();
            token.allowances_of(&token_holder)
        }),
        Err(_) => Vec::new(),
    }
}

#[update(name = "transferFrom")]
#[candid_method(update, rename = "transferFrom")]
async fn transfer_from(
    spender_sub_account: Option<Subaccount>,
    from: String,
    to: String,
    value: Nat,
    nonce: Option<u64>,
) -> TransactionResult {
    let caller = api::caller();
    let now = api::time();
    let spender = TokenHolder::new(caller, spender_sub_account);

    match from.parse::<TokenHolder>() {
        Ok(from_token_holder) => match to.parse::<TokenHolder>() {
            Ok(to_token_holder) => {
                // exec before-transfer check :before_token_sending
                match before_token_sending(&from_token_holder, &to_token_holder, &value) {
                    Err(e) => return TransactionResult::Err(e),
                    _ => {}
                };
                match TOKEN.with(|token| {
                    let mut token = token.borrow_mut();
                    token.transfer_from(
                        &caller,
                        &from_token_holder,
                        &spender,
                        &to_token_holder,
                        value.clone(),
                        nonce,
                        now,
                    )
                }) {
                    Ok(tx_index) => {
                        // exec auto scaling strategy
                        TransactionResult::Ok(TransactionResponse {
                            tx_id: encode_tx_id(api::id(), tx_index),
                            error: match exec_auto_scaling_strategy().await {
                                Err(e) => Some(e),
                                _ => None,
                            },
                        })
                    }
                    Err(e) => TransactionResult::Err(e.into()),
                }
            }
            _ => TransactionResult::Err(DFTError::InvalidArgFormatTo.into()),
        },
        _ => TransactionResult::Err(DFTError::InvalidArgFormatFrom.into()),
    }
}

#[update(name = "transfer")]
#[candid_method(update, rename = "transfer")]
async fn transfer(
    from_sub_account: Option<Subaccount>,
    to: String,
    value: Nat,
    nonce: Option<u64>,
) -> TransactionResult {
    let caller = api::caller();
    let now = api::time();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let receiver_parse_result = to.parse::<TokenReceiver>();

    match receiver_parse_result {
        Ok(receiver) => {
            //exec before-transfer check
            match before_token_sending(&transfer_from, &receiver, &value) {
                Err(e) => return TransactionResult::Err(e),
                _ => {}
            };
            //transfer token
            match TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.transfer(
                    &caller,
                    &transfer_from,
                    &receiver,
                    value.clone(),
                    nonce,
                    now,
                )
            }) {
                Ok(tx_index) => TransactionResult::Ok(TransactionResponse {
                    tx_id: encode_tx_id(api::id(), tx_index),
                    error: match exec_auto_scaling_strategy().await {
                        Ok(_) => None,
                        Err(e) => Some(e),
                    },
                }),
                Err(e) => TransactionResult::Err(e.into()),
            }
        }
        _ => TransactionResult::Err(DFTError::InvalidArgFormatTo.into()),
    }
}

#[query(name = "tokenInfo")]
#[candid_method(query, rename = "tokenInfo")]
fn get_token_info() -> TokenInfo {
    TOKEN.with(|token| {
        let token = token.borrow();
        let mut token_info = token.token_info();
        token_info.cycles = api::canister_balance();
        token_info
    })
}

#[query(name = "transactionByIndex")]
#[candid_method(query, rename = "transactionByIndex")]
fn transaction_by_index(tx_index: Nat) -> TxRecordResult {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.transaction_by_index(&tx_index).into()
    })
}

#[query(name = "lastTransactions")]
#[candid_method(query, rename = "lastTransactions")]
fn last_transactions(count: usize) -> TxRecordListResult {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.last_transactions(count).into()
    })
}

#[query(name = "transactionById")]
#[candid_method(query, rename = "transactionById")]
fn transaction_by_id(tx_id: String) -> TxRecordResult {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.transaction_by_id(&tx_id).into()
    })
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}

// do something before sending
fn before_token_sending(
    _transfer_from: &TokenHolder,
    _receiver: &TokenReceiver,
    _value: &Nat,
) -> ActorResult<()> {
    Ok(())
}
