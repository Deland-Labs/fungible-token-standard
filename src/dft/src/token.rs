#![allow(dead_code)]
use crate::extends;
use crate::storage;
use crate::types;
use crate::utils;
/**
 * Module     : token.rs
 * Copyright  : 2021 Deland Team
 * License    : Apache 2.0 with LLVM Exception
 * Maintainer : Deland Team (https://deland.one)
 * Stability  : Experimental
 */
use candid::{candid_method, IDLProg};
use dfn_candid::{candid, candid_one};
use dfn_core::{
    api::{call_bytes_with_cleanup, call_with_cleanup, Funds},
    over, over_async,
};
use ic_types::{CanisterId, PrincipalId};
use ledger_canister::account_identifier::AccountIdentifier;
use std::collections::HashMap;
use std::string::String;
use types::{
    ApproveResult, BurnResult, CallData, ExtendData, Fee, KeyValuePair, MetaData, TokenHolder,
    TokenReceiver, TransactionId, TransferFrom, TransferResult, TxRecord,
};

type Balances = HashMap<TokenHolder, u128>;
type Allowances = HashMap<TokenHolder, HashMap<TokenHolder, u128>>;

static mut OWNER: PrincipalId = PrincipalId::new(0, [0u8; 29]);
static mut NAME: &str = "";
static mut SYMBOL: &str = "";
static mut DECIMALS: u8 = 0;
static mut TOTAL_SUPPLY: u128 = 0;
static mut FEE: types::Fee = types::Fee::Fixed(0);
static mut TOTAL_FEE: u128 = 0;
// 256 * 256
static mut LOGO: Vec<u8> = Vec::new();
// 256 * 256
static mut TXS: Vec<TxRecord> = Vec::new();

// transferFee = amount * rate / 10.pow(FEE_RATE_DECIMALS)
const FEE_RATE_DECIMALS: u8 = 6u8;

// #[export_name = "canister_init"]
// #[candid_method(init)]
// fn init(
//     subaccount: Option<String>,
//     logo: Vec<u8>,
//     name: String,
//     symbol: String,
//     decimals: u8,
//     total_supply: u128,
// ) {
//     unsafe {
//         LOGO = logo;
//         NAME = Box::leak(name.into_boxed_str());
//         SYMBOL = Box::leak(symbol.into_boxed_str());
//         DECIMALS = decimals;
//         TOTAL_SUPPLY = total_supply;
//         OWNER = dfn_core::api::caller();
//         let call_from = parse_to_token_holder(OWNER, subaccount).unwrap();
//         let balances = storage::get_mut::<Balances>();
//         balances.insert(call_from, TOTAL_SUPPLY);

//         TXS.push(TxRecord::Init(
//             OWNER,
//             OWNER,
//             decimals,
//             total_supply,
//             dfn_core::api::ic0::time(),
//         ));
//     }
// }

#[export_name = "canister_query meta"]
fn get_meta_data() {
    over(candid, |()| -> MetaData { _get_meta_data() })
}

#[candid_method(query, rename = "meta")]
fn _get_meta_data() -> MetaData {
    unsafe {
        let meta = MetaData {
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            decimals: DECIMALS,
            total_supply: TOTAL_SUPPLY,
            fee: FEE.clone(),
        };

        dfn_core::api::print(format!("meta is {:#?}", meta));
        meta
    }
}

#[export_name = "canister_query extend"]
fn get_extend_data() {
    over(candid, |()| -> Vec<KeyValuePair> { _get_extend_data() })
}

#[candid_method(query, rename = "extend")]
fn _get_extend_data() -> Vec<KeyValuePair> {
    let extend_data_store = storage::get::<ExtendData>();
    let mut return_vec: Vec<KeyValuePair> = Vec::new();
    for (k, v) in extend_data_store.iter() {
        return_vec.push(KeyValuePair {
            k: k.to_string(),
            v: v.to_string(),
        });
    }
    return_vec
}
#[export_name = "canister_update updateExtend"]
fn update_extend_data() {
    over(candid_one, |extend_data: Vec<KeyValuePair>| -> bool {
        _update_extend_data(extend_data)
    })
}

#[candid_method(update, rename = "updateExtend")]
fn _update_extend_data(extend_data: Vec<KeyValuePair>) -> bool {
    let extend_data_store = storage::get_mut::<ExtendData>();
    for kv_pair in extend_data.iter() {
        extend_data_store.insert(kv_pair.k.clone(), kv_pair.v.clone());
    }
    true
}

#[export_name = "canister_query logo"]
fn logo() {
    over(candid, |()| -> Vec<u8> { _logo() })
}
#[candid_method(query, rename = "logo")]
fn _logo() -> Vec<u8> {
    unsafe { LOGO.clone() }
}

#[export_name = "canister_update updateLogo"]
fn update_logo() {
    over(candid_one, _update_logo)
}

#[candid_method(update, rename = "updateLogo")]
fn _update_logo(logo: Vec<u8>) -> bool {
    unsafe { LOGO = logo }
    true
}

#[export_name = "canister_query balanceOf"]
fn balance_of() {
    over(candid_one, _balance_of)
}

#[candid_method(query, rename = "balanceOf")]
fn _balance_of(holder: String) -> u128 {
    let token_holder_parse_result = holder.parse::<TokenHolder>();

    let balance = match token_holder_parse_result {
        Ok(token_holder) => _inner_balance_of(&token_holder),
        _ => 0,
    };
    dfn_core::api::print(format!("get account balance is {}", balance));
    balance
}

fn _inner_balance_of(holder: &TokenHolder) -> u128 {
    let balances = storage::get::<Balances>();
    match balances.get(holder) {
        Some(balance) => *balance,
        None => 0,
    }
}

#[export_name = "canister_query allowance"]
fn allowance() {
    over(candid, |(owner, spender): (String, String)| {
        _allowance(owner, spender)
    })
}

#[candid_method(query, rename = "allowance")]
fn _allowance(owner: String, spender: String) -> u128 {
    let token_holder_owner_parse_result = owner.parse::<TokenHolder>();
    let token_holder_spender_parse_result = spender.parse::<TokenHolder>();

    let allowance: u128 = match token_holder_owner_parse_result {
        Ok(token_holder_owner) => match token_holder_spender_parse_result {
            Ok(token_holder_spender) => {
                _inner_allowance(&token_holder_owner, &token_holder_spender)
            }
            _ => 0u128,
        },
        _ => 0u128,
    };

    dfn_core::api::print(format!("get allowance is {}", allowance));
    allowance
}

fn _inner_allowance(owner: &TokenHolder, spender: &TokenHolder) -> u128 {
    let allowances = storage::get::<Allowances>();
    match allowances.get(&owner) {
        Some(inner) => match inner.get(&spender) {
            Some(value) => *value,
            None => 0u128,
        },
        None => 0u128,
    }
}

#[export_name = "canister_update approve"]
fn approve() {
    over_async(
        candid,
        |(owner_sub_account, spender, value, call_data): (
            Option<String>,
            String,
            u128,
            Option<CallData>,
        )| { _approve(owner_sub_account, spender, value, call_data) },
    )
}

#[candid_method(update, rename = "approve")]
async fn _approve(
    owner_sub_account: Option<String>,
    spender: String,
    value: u128,
    call_data: Option<CallData>,
) -> ApproveResult {
    let owner = dfn_core::api::caller();
    let owner_parse_result = parse_to_token_holder(owner, owner_sub_account);
    let spender_parse_result = spender.parse::<TokenHolder>();

    if let Ok(owner_holder) = owner_parse_result {
        if let Ok(spender_holder) = spender_parse_result {
            let allowances_read = storage::get::<Allowances>();
            match allowances_read.get(&owner_holder) {
                Some(inner) => {
                    let mut temp = inner.clone();
                    temp.insert(spender_holder.clone(), value);
                    let allowances = storage::get_mut::<Allowances>();
                    allowances.insert(owner_holder.clone(), temp);
                }
                None => {
                    let mut inner = HashMap::new();
                    inner.insert(spender_holder.clone(), value);
                    let allowances = storage::get_mut::<Allowances>();
                    allowances.insert(owner_holder.clone(), inner);
                }
            };
            // execute call
            let execute_call_result = _execute_call(&spender_holder, call_data).await;

            if let Err(e) = execute_call_result {
                // approve succeed ,bu call failed
                return ApproveResult::Ok(e);
            };
        }
    }
    ApproveResult::Ok
}

#[export_name = "canister_update transferFrom"]
fn transfer_from() {
    over_async(
        candid,
        |(spender_sub_account, from, to, value): (Option<String>, String, String, u128)| {
            _transfer_from(spender_sub_account, from, to, value)
        },
    )
}

#[candid_method(update, rename = "transferFrom")]
async fn _transfer_from(
    spender_sub_account: Option<String>,
    from: String,
    to: String,
    value: u128,
) -> TransferResult {
    let spender_principal_id = dfn_core::api::caller();
    let spender_parse_result = parse_to_token_holder(spender_principal_id, spender_sub_account);

    let from_parse_result = from.parse::<TokenHolder>();
    let to_parse_result = to.parse::<TokenHolder>();
    let fee = _calc_fee(value);

    match spender_parse_result {
        Ok(spender) => match from_parse_result {
            Ok(from_token_holder) => match to_parse_result {
                Ok(to_token_holder) => {
                    let mut from_balance = _inner_balance_of(&from_token_holder);
                    let mut from_allowance = _inner_allowance(&from_token_holder, &spender);
                    if from_allowance < value + fee {
                        return TransferResult::Err(types::Error::InsufficientAllowance);
                    } else if from_balance < value + fee {
                        return TransferResult::Err(types::Error::InsufficientBalance);
                    }

                    let balances = storage::get_mut::<Balances>();

                    // before transfer hook
                    let before_sending_check_result =
                        _on_token_sending(&from_token_holder, &to_token_holder, &value).await;

                    if let Err(e) = before_sending_check_result {
                        return TransferResult::Err(e);
                    }
                    // reload the balance after async call (_on_token_sending)
                    from_balance = _inner_balance_of(&from_token_holder);
                    // reload the allowance after async call (_on_token_sending)
                    from_allowance = _inner_allowance(&from_token_holder, &spender);
                    // recheck balance & allowance
                    if from_allowance < value {
                        return TransferResult::Err(types::Error::InsufficientAllowance);
                    } else if from_balance < value {
                        return TransferResult::Err(types::Error::InsufficientBalance);
                    }
                    let to_balance = _inner_balance_of(&to_token_holder);
                    balances.insert(from_token_holder.clone(), from_balance - value - fee);
                    balances.insert(to_token_holder.clone(), to_balance + value);
                    let allowances_read = storage::get::<Allowances>();
                    match allowances_read.get(&from_token_holder) {
                        Some(inner) => {
                            let mut temp = inner.clone();
                            temp.insert(spender, from_allowance - value);
                            let allowances = storage::get_mut::<Allowances>();
                            allowances.insert(from_token_holder.clone(), temp);
                        }
                        None => {
                            //revert balance and allowance
                            assert!(false);
                        }
                    }
                    // after transfer hook
                    let after_token_send_notify_result =
                        _on_token_received(&from_token_holder, &to_token_holder, &value).await;

                    if let Err(e) = after_token_send_notify_result {
                        return TransferResult::Ok(1u128, Some(vec![e]));
                    } else {
                        return TransferResult::Ok(1u128, None);
                    }
                }
                _ => TransferResult::Err(types::Error::InvalidReceiver),
            },
            _ => TransferResult::Err(types::Error::InvalidTokenHolder),
        },
        _ => TransferResult::Err(types::Error::InvalidSpender),
    }
}

#[export_name = "canister_update transfer"]
fn transfer() {
    over_async(
        candid,
        |(from_sub_account, to, amount, call_data): (
            Option<String>,
            String,
            u128,
            Option<CallData>,
        )| { _transfer(from_sub_account, to, amount, call_data) },
    )
}

#[candid_method(update, rename = "transfer")]
async fn _transfer(
    from_sub_account: Option<String>,
    to: String,
    value: u128,
    call_data: Option<CallData>,
) -> TransferResult {
    let from = dfn_core::api::caller();
    let transfer_from_parse_result = parse_to_token_holder(from, from_sub_account);
    let receiver_parse_result = to.parse::<TokenReceiver>();
    let fee = _calc_fee(value);
    match transfer_from_parse_result {
        Ok(transfer_from) => {
            let mut from_balance = _inner_balance_of(&transfer_from);
            dfn_core::api::print(format!(
                "from account balance is {}",
                from_balance.to_string()
            ));

            if from_balance < value + fee {
                return TransferResult::Err(types::Error::InsufficientBalance);
            }

            match receiver_parse_result {
                Ok(receiver) => {
                    let to_balance = _inner_balance_of(&receiver);
                    let balances = storage::get_mut::<Balances>();

                    // before transfer hook
                    let before_sending_check_result =
                        _on_token_sending(&transfer_from, &receiver, &value).await;

                    if let Err(e) = before_sending_check_result {
                        return TransferResult::Err(e);
                    }
                    // reload balance after outside call (_on_token_sending)
                    from_balance = _inner_balance_of(&transfer_from);

                    if from_balance < value + fee {
                        return TransferResult::Err(types::Error::InsufficientBalance);
                    }

                    balances.insert(transfer_from.clone(), from_balance - value - fee);
                    balances.insert(receiver.clone(), to_balance + value);

                    unsafe {
                        TXS.push(TxRecord::Transfer(
                            from,
                            transfer_from.clone(),
                            receiver.clone(),
                            value,
                            fee,
                            dfn_core::api::ic0::time(),
                        ));
                        TOTAL_FEE += fee;
                    }

                    let mut errors: Vec<types::Error> = Vec::new();

                    // after transfer hook
                    let after_token_send_notify_result =
                        _on_token_received(&transfer_from, &receiver, &value).await;

                    if let Err(e) = after_token_send_notify_result {
                        errors.push(e);
                    };

                    // execute call
                    let execute_call_result = _execute_call(&receiver, call_data).await;

                    if let Err(e) = execute_call_result {
                        errors.push(e);
                    };
                    TransferResult::Ok(1u128, None)
                }
                Err(e) => return TransferResult::Err(e),
            }
        }
        Err(e) => return TransferResult::Err(e),
    }
}

#[export_name = "canister_update burn"]
fn burn() {
    over(
        candid_one,
        |(from_sub_account, amount): (Option<String>, u128)| _burn(from_sub_account, amount),
    )
}

#[candid_method(update, rename = "burn")]
fn _burn(from_sub_account: Option<String>, value: u128) -> BurnResult {
    let from = dfn_core::api::caller();
    let transfer_from_parse_result = parse_to_token_holder(from, from_sub_account);
    let fee = _calc_fee(value);

    if fee > value {
        return BurnResult::Err(types::Error::QuantityTooSmall);
    }

    match transfer_from_parse_result {
        Ok(transfer_from) => {
            let from_balance = _inner_balance_of(&transfer_from);

            if from_balance < value {
                return BurnResult::Err(types::Error::InsufficientBalance);
            }

            let balances = storage::get_mut::<Balances>();
            balances.insert(transfer_from.clone(), from_balance - value);
            unsafe {
                TXS.push(TxRecord::Burn(
                    from,
                    transfer_from.clone(),
                    value,
                    dfn_core::api::ic0::time(),
                ));
            }
            BurnResult::Ok
        }
        _ => BurnResult::Err(types::Error::InsufficientBalance),
    }
}

#[export_name = "canister_query supportedInterface"]
fn supported_interface() {
    over(candid_one, _supported_interface)
}

#[candid_method(query, rename = "supportedInterface")]
fn _supported_interface(interface: String) -> bool {
    let verify_service_desc = format!("service:{{ {0};}}", interface);
    let verify_ast_result = verify_service_desc.parse::<IDLProg>();

    match verify_ast_result {
        Ok(verify_ast) => {
            let verify_pretty: String = candid::parser::types::to_pretty(&verify_ast, 80);
            let verify_pretty_sub: String =
                verify_pretty.replace("service : { ", "").replace(" }", "");

            let origin_did = __export_did_tmp_();
            let origin_ast: IDLProg = origin_did.parse().unwrap();
            let origin_pretty: String = candid::parser::types::to_pretty(&origin_ast, 80);
            origin_pretty.contains(&verify_pretty_sub)
        }
        _ => false,
    }
}

fn parse_to_token_holder(
    from: PrincipalId,
    from_sub_account: Option<String>,
) -> Result<TransferFrom, types::Error> {
    if !utils::is_canister(&from) {
        match from_sub_account {
            Some(s) => {
                let account_identity = &s.parse::<AccountIdentifier>();
                match account_identity {
                    Ok(_ai) => Ok(TransferFrom::Account(*_ai)),
                    _ => Err(types::Error::InvalidSubaccount),
                }
            }
            _ => Ok(TransferFrom::Principal(from)),
        }
    } else {
        let cid = CanisterId::new(from).unwrap();
        Ok(TransferFrom::Canister(cid))
    }
}

async fn _on_token_sending(
    transfer_from: &TokenHolder,
    receiver: &TokenReceiver,
    _value: &u128,
) -> Result<bool, types::Error> {
    let supported_interface_method_name = "supportedInterface";
    let on_token_sending_method_name = "on_token_sending";
    let on_token_sending_method_sig =
        "on_token_sending:(TransferFrom,TokenReceiver,nat128)->(bool)query";
    // check transfer from
    if let TransferFrom::Canister(tf_cid) = transfer_from {
        let support_res: Result<(bool,), _> = call_with_cleanup(
            *tf_cid,
            supported_interface_method_name,
            dfn_candid::candid_one,
            on_token_sending_method_sig,
        )
        .await;

        if let Ok((_support,)) = support_res {
            if _support {
                let _check_res: Result<(bool,), _> = call_with_cleanup(
                    *tf_cid,
                    on_token_sending_method_name,
                    dfn_candid::candid_multi_arity,
                    (transfer_from, receiver, _value),
                )
                .await;

                match _check_res {
                    Ok((is_sending_succeed,)) => {
                        if !is_sending_succeed {
                            return Err(types::Error::RejectedByHolder);
                        }
                    }
                    _ => return Err(types::Error::RejectedByHolder),
                }
            }
        }
    }

    // check receiver
    if let TokenReceiver::Canister(r_cid) = receiver {
        let support_res: Result<(bool,), _> = call_with_cleanup(
            *r_cid,
            supported_interface_method_name,
            dfn_candid::candid_one,
            on_token_sending_method_sig,
        )
        .await;

        if let Ok((_support,)) = support_res {
            if _support {
                let _check_res: Result<(bool,), _> = call_with_cleanup(
                    *r_cid,
                    on_token_sending_method_name,
                    dfn_candid::candid_multi_arity,
                    (transfer_from, receiver, _value),
                )
                .await;

                match _check_res {
                    Ok((is_sending_succeed,)) => {
                        if !is_sending_succeed {
                            return Err(types::Error::RejectedByReceiver);
                        }
                    }
                    _ => return Err(types::Error::RejectedByReceiver),
                }
            }
        }
    }

    Ok(true)
}

// call it after transfer, notify receiver with (from,value)
async fn _on_token_received(
    transfer_from: &TransferFrom,
    receiver: &TokenReceiver,
    _value: &u128,
) -> Result<bool, types::Error> {
    let supported_interface_method_name = "supportedInterface";
    let on_token_received_method_name = "on_token_received";
    let on_token_received_method_sig = "on_token_received:(TransferFrom,nat128)->(bool)query";

    // check receiver
    if let TokenHolder::Canister(cid) = receiver {
        let support_res: Result<(bool,), _> = call_with_cleanup(
            *cid,
            supported_interface_method_name,
            dfn_candid::candid_one,
            on_token_received_method_sig,
        )
        .await;

        if let Ok((_support,)) = support_res {
            if _support {
                let _check_res: Result<(bool,), _> = call_with_cleanup(
                    *cid,
                    on_token_received_method_name,
                    dfn_candid::candid_multi_arity,
                    (transfer_from, _value),
                )
                .await;

                dfn_core::api::print("notify executed!");

                match _check_res {
                    Ok((is_notify_succeed,)) => {
                        if !is_notify_succeed {
                            return Err(types::Error::NotifyFailed);
                        }
                    }
                    _ => return Err(types::Error::NotifyFailed),
                }
            }
        }
    }
    Ok(true)
}

async fn _execute_call(
    receiver: &TokenReceiver,
    _call_data: Option<CallData>,
) -> Result<bool, types::Error> {
    if let TokenHolder::Canister(cid) = receiver {
        match _call_data {
            Some(call_data) => {
                match call_bytes_with_cleanup(
                    *cid,
                    &call_data.method,
                    &call_data.args,
                    Funds::zero(),
                )
                .await
                {
                    Ok(_) => {
                        return Ok(true);
                    }
                    _ => return Err(types::Error::CallFailed),
                };
            }
            _ => {}
        }
    }

    Ok(true)
}

fn _calc_fee(value: u128) -> u128 {
    unsafe {
        let div_by = (10 as u128).pow(FEE_RATE_DECIMALS as u32);
        match FEE {
            Fee::Fixed(_fixed) => _fixed,
            Fee::Rate(_rate) => value * (_rate as u128) / div_by,
            Fee::RateWithLowestLimit(_lowest, _rate) => {
                std::cmp::max(_lowest, value * (_rate as u128) / div_by)
            }
        }
    }
}

candid::export_service!();

#[export_name = "canister_query __export_did_tmp"]
fn __export_did_tmp() {
    over(candid, |()| -> String { __export_did_tmp_() })
}

#[candid_method(query, rename = "__export_did_tmp")]
fn __export_did_tmp_() -> String {
    __export_service()
}
