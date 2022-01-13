extern crate dft_types;
extern crate dft_utils;

use candid::{candid_method, decode_args};
use dft_standard::token::TokenStandard;
use dft_types::*;
use dft_utils::*;
use ic_cdk::{
    api,
    export::{candid::Nat, Principal},
};
use ic_cdk_macros::*;
use std::{collections::HashMap, string::String};

use crate::{auto_scaling_storage::exec_auto_scaling_strategy, state::TOKEN};

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
        let _ = token._mint(&real_caller, &owner_holder, total_supply_, api::time());
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

#[update(name = "setOwner")]
#[candid_method(update, rename = "setOwner")]
fn set_owner(owner: Principal) -> ActorResult<bool> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.set_owner(&api::caller(), owner)?;
        Ok(true)
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

#[update(name = "setDesc")]
#[candid_method(update, rename = "setDesc")]
fn set_desc_info(desc_data: Vec<(String, String)>) -> ActorResult<bool> {
    // convert desc data to hashmap
    let mut desc_info = HashMap::new();
    for (key, value) in desc_data {
        desc_info.insert(key, value);
    }
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        to_actor_result(token.set_desc(&api::caller(), desc_info))
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

#[update(name = "setLogo")]
#[candid_method(update, rename = "setLogo")]
fn set_logo(logo: Vec<u8>) -> ActorResult<bool> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        to_actor_result(token.set_logo(&api::caller(), logo))
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
    call_data: Option<CallData>,
) -> ActorResult<TransactionResponse> {
    let caller = api::caller();
    let owner_holder = TokenHolder::new(caller.clone(), owner_sub_account);
    match spender.parse::<TokenHolder>() {
        Ok(spender_holder) => {
            let tx_index = TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.approve(&caller, &owner_holder, &spender_holder, value, api::time())
            })?;
            let tx_id = encode_tx_id(api::id(), tx_index);

            let mut errors: Vec<ActorError> = vec![];
            match exec_auto_scaling_strategy().await {
                Ok(_) => (),
                Err(e) => {
                    errors.push(e);
                }
            }

            if let Some(data) = call_data {
                // execute call
                let execute_call_result = execute_call(&spender_holder, data).await;
                if let Err(emsg) = execute_call_result {
                    // approve succeed ,bu call failed
                    errors.push(emsg);
                }
            };
            Ok(TransactionResponse {
                tx_id: tx_id,
                error: if errors.len() > 0 { Some(errors) } else { None },
            })
        }
        Err(_) => Err(DFTError::InvalidSpender.into()),
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
) -> ActorResult<TransactionResponse> {
    let caller = api::caller();
    let now = api::time();
    let spender = TokenHolder::new(caller, spender_sub_account);

    match from.parse::<TokenHolder>() {
        Ok(from_token_holder) => match to.parse::<TokenHolder>() {
            Ok(to_token_holder) => {
                // exec before-transfer check
                before_token_sending(&from_token_holder, &to_token_holder, &value)?;
                let tx_index = TOKEN.with(|token| {
                    let mut token = token.borrow_mut();
                    token.transfer_from(
                        &caller,
                        &from_token_holder,
                        &spender,
                        &to_token_holder,
                        value.clone(),
                        now,
                    )
                })?;
                let mut errors: Vec<ActorError> = vec![];
                // exec auto scaling strategy
                match exec_auto_scaling_strategy().await {
                    Err(e) => errors.push(e),
                    _ => {}
                };
                // exec after-transfer notify
                match on_token_received(&from_token_holder, &to_token_holder, &value).await {
                    Err(e) => errors.push(e),
                    _ => (),
                };
                Ok(TransactionResponse {
                    tx_id: encode_tx_id(api::id(), tx_index),
                    error: if errors.len() > 0 { Some(errors) } else { None },
                })
            }
            _ => Err(DFTError::InvalidArgFormatTo.into()),
        },
        _ => Err(DFTError::InvalidArgFormatFrom.into()),
    }
}

#[update(name = "transfer")]
#[candid_method(update, rename = "transfer")]
async fn transfer(
    from_sub_account: Option<Subaccount>,
    to: String,
    value: Nat,
    call_data: Option<CallData>,
) -> ActorResult<TransactionResponse> {
    let caller = api::caller();
    let now = api::time();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let receiver_parse_result = to.parse::<TokenReceiver>();

    match receiver_parse_result {
        Ok(receiver) => {
            //exec before-transfer check
            before_token_sending(&transfer_from, &receiver, &value)?;
            let mut errors: Vec<ActorError> = Vec::new();
            //transfer token
            let tx_index = TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.transfer(&caller, &transfer_from, &receiver, value.clone(), now)
            })?;

            //exec auto-scaling storage strategy
            match exec_auto_scaling_strategy().await {
                Ok(_) => (),
                Err(e) => {
                    errors.push(e);
                }
            };
            // exec after-transfer notify
            match on_token_received(&transfer_from, &receiver, &value).await {
                Err(e) => errors.push(e),
                _ => (),
            };
            // execute call
            if let Some(_call_data) = call_data {
                // execute call
                let execute_call_result = execute_call(&receiver, _call_data).await;
                if let Err(emsg) = execute_call_result {
                    errors.push(emsg);
                };
            }
            return Ok(TransactionResponse {
                tx_id: encode_tx_id(api::id(), tx_index),
                error: if errors.len() > 0 { Some(errors) } else { None },
            });
        }
        _ => Err(DFTError::InvalidArgFormatTo.into()),
    }
}

#[update(name = "setFee")]
#[candid_method(update, rename = "setFee")]
fn set_fee(fee: Fee) -> ActorResult<bool> {
    let caller = api::caller();
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        to_actor_result(token.set_fee(&caller, fee))
    })
}

#[query(name = "setFeeTo")]
#[candid_method(update, rename = "setFeeTo")]
fn set_fee_to(fee_to: String) -> ActorResult<bool> {
    match fee_to.parse::<TokenReceiver>() {
        Ok(holder) => TOKEN.with(|token| {
            let mut token = token.borrow_mut();
            to_actor_result(token.set_fee_to(&api::caller(), holder))
        }),
        Err(_) => Err(DFTError::InvalidArgFormatFeeTo.into()),
    }
}

#[query(name = "tokenInfo")]
#[candid_method(query, rename = "tokenInfo")]
fn get_token_info() -> TokenInfo {
    // let mut token_info = TOKEN.read().unwrap().token_info();
    // token_info.cycles = api::canister_balance();
    // token_info
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
fn last_transactions(count: usize) -> ActorResult<Vec<TxRecord>> {
    TOKEN.with(|token| {
        let token = token.borrow();
        to_actor_result(token.last_transactions(count))
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

// do something becore sending
fn before_token_sending(
    _transfer_from: &TokenHolder,
    _receiver: &TokenReceiver,
    _value: &Nat,
) -> ActorResult<()> {
    Ok(())
}

// call it after transfer, notify receiver with (from,value)
async fn on_token_received(
    transfer_from: &TransferFrom,
    receiver: &TokenReceiver,
    _value: &Nat,
) -> ActorResult<bool> {
    let get_did_method_name = "__get_candid_interface_tmp_hack";
    let on_token_received_method_name = "onTokenReceived";
    let on_token_received_method_sig = "onTokenReceived:(TransferFrom,nat)->(bool)query";

    // check receiver
    if let TokenHolder::Principal(cid) = receiver {
        if is_canister(cid) {
            let did_res: Result<(String,), _> =
                api::call::call(*cid, get_did_method_name, ()).await;

            if let Ok((did,)) = did_res {
                let _support = is_support_interface(did, on_token_received_method_sig.to_string());

                if _support {
                    let _check_res: Result<(bool,), _> = api::call::call(
                        *cid,
                        on_token_received_method_name,
                        (transfer_from, _value),
                    )
                    .await;

                    match _check_res {
                        Ok((is_notify_succeed,)) => {
                            if !is_notify_succeed {
                                return Err(DFTError::NotificationFailed.into());
                            } else {
                                return Ok(true);
                            }
                        }
                        _ => return Err(DFTError::NotificationFailed.into()),
                    }
                }
            }
            return Err(DFTError::NotificationFailed.into());
        }
    }
    Ok(true)
}

async fn execute_call(receiver: &TokenReceiver, _call_data: CallData) -> ActorResult<bool> {
    if let TokenHolder::Principal(cid) = receiver {
        if is_canister(cid) {
            let call_result: Result<Vec<u8>, (api::call::RejectionCode, String)> =
                api::call::call_raw(*cid, &_call_data.method, _call_data.args, 0).await;
            match call_result {
                Ok(bytes) => {
                    let r: (bool, String) = decode_args(&bytes).unwrap();
                    if r.0 {
                        return Ok(r.0);
                    } else {
                        return Err(DFTError::CallFailed { detail: r.1 }.into());
                    }
                }
                Err(e) => return Err(DFTError::CallFailed { detail: e.1 }.into()),
            };
        }
    }
    Ok(true)
}
