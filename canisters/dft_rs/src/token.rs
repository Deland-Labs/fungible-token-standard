/**
 * Module     : token.rs
 * Copyright  : 2021 Deland-Labs Team
 * License    : Apache 2.0 with LLVM Exception
 * Maintainer : Deland Team (https://delandlabs.com)
 * Stability  : Experimental
 */
use crate::extends;
use crate::ic_management::{
    create_canister, get_canister_status, install_canister, CanisterIdRecord, CanisterSettings,
    CreateCanisterArgs,
};
use crate::types::{message::*, *};
use crate::utils::*;
use candid::{candid_method, decode_args, encode_args};
use ic_cdk::{
    api,
    export::{candid::Nat, Principal},
    storage,
};
use ic_cdk_macros::*;
use num_bigint::BigUint;
use std::cmp;
use std::collections::HashMap;
use std::ops::{Div, Mul};
use std::string::String;
use std::sync::RwLock;

// transferFee = amount * rate / 10.pow(FEE_RATE_DECIMALS)
const MAX_TXS_CACHE_IN_DFT: usize = 1;
const FEE_RATE_DECIMALS: u8 = 8u8;
const MAX_HEAP_MEMORY_SIZE: u32 = 4294967295u32; // 4G
const CYCLES_PER_TOKEN: u64 = 2000_000_000_000; // 2T

lazy_static! {
    static ref NAT_ZERO: Nat = Nat::from(0);
    static ref TOTAL_SUPPLY: RwLock<Nat> = RwLock::new(Nat::from(0));
    static ref FEE: RwLock<Fee> = RwLock::new(Fee {
        lowest: Nat::from(0),
        rate: Nat::from(0),
    });
    static ref TX_ID_CURSOR: RwLock<Nat> = RwLock::new(Nat::from(0));
}

static mut OWNER: Principal = Principal::anonymous();
static mut NAME: &str = "";
static mut SYMBOL: &str = "";
static mut DECIMALS: u8 = 0;
static mut LOGO: Vec<u8> = Vec::new(); // 256 * 256
static mut FEE_TO: TokenHolder = TokenHolder::Principal(Principal::anonymous());

#[init]
fn canister_init(
    sub_account: Option<Subaccount>,
    logo: Option<Vec<u8>>,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: Nat,
    fee: Fee,
    owner: Option<Principal>,
) {
    let mut caller = api::caller();
    // When using proxy tools to issue tokens, should use the parameter owner instead of api::caller() as the real caller
    if let Some(real_caller) = owner {
        caller = real_caller;
    };
    unsafe {
        OWNER = caller;
        LOGO = if logo.is_some() {
            logo.unwrap()
        } else {
            [].to_vec()
        };
        let mut rw_total_supply = TOTAL_SUPPLY.write().unwrap();
        let mut rw_fee = FEE.write().unwrap();
        NAME = Box::leak(name.into_boxed_str());
        SYMBOL = Box::leak(symbol.into_boxed_str());
        DECIMALS = decimals;
        *rw_total_supply = total_supply;
        *rw_fee = fee;
        let call_from = TokenHolder::new(caller, sub_account);
        FEE_TO = call_from.clone();
        ic_cdk::print(format!("caller is {}", caller.to_text()));
        match call_from {
            TokenHolder::Account(a) => ic_cdk::print(format!("init : account is {}", a.to_hex())),
            TokenHolder::Principal(p) => {
                ic_cdk::print(format!("init : account is {}", p.to_text()))
            }
        };
        let balances = storage::get_mut::<Balances>();
        balances.insert(call_from.clone(), rw_total_supply.clone());
    }
}

#[update(name = "owner")]
#[candid_method(update, rename = "owner")]
fn owner() -> Principal {
    unsafe { OWNER }
}

#[update(name = "setOwner")]
#[candid_method(update, rename = "setOwner")]
fn set_owner(owner: Principal) -> bool {
    _only_owner();
    unsafe {
        OWNER = owner;
    }
    true
}

#[query(name = "name")]
#[candid_method(query, rename = "name")]
fn get_name() -> String {
    unsafe { NAME.to_string() }
}

#[query(name = "symbol")]
#[candid_method(query, rename = "symbol")]
fn get_symbol() -> String {
    unsafe { SYMBOL.to_string() }
}

#[query(name = "decimals")]
#[candid_method(query, rename = "decimals")]
fn get_decimals() -> u8 {
    unsafe { DECIMALS }
}

#[query(name = "totalSupply")]
#[candid_method(query, rename = "totalSupply")]
fn get_total_supply() -> Nat {
    (*TOTAL_SUPPLY.read().unwrap()).clone()
}

#[query(name = "fee")]
#[candid_method(query, rename = "fee")]
fn get_fee_setting() -> Fee {
    (*FEE.read().unwrap()).clone()
}

#[query(name = "meta")]
#[candid_method(query, rename = "meta")]
fn get_meta_data() -> MetaData {
    unsafe {
        let meta = MetaData {
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            decimals: DECIMALS,
            total_supply: (*TOTAL_SUPPLY.read().unwrap()).clone(),
            fee: (*FEE.read().unwrap()).clone(),
        };

        ic_cdk::print(format!("meta is {:#?}", meta));
        meta
    }
}

#[query(name = "extend")]
#[candid_method(query, rename = "extend")]
fn get_extend_data() -> Vec<KeyValuePair> {
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

#[update(name = "setExtend")]
#[candid_method(update, rename = "setExtend")]
fn set_extend_data(extend_data: Vec<KeyValuePair>) -> bool {
    _only_owner();
    let extend_data_store = storage::get_mut::<ExtendData>();
    for kv_pair in extend_data.iter() {
        if extends::EXTEND_KEYS.contains(&kv_pair.k.as_str()) {
            extend_data_store.insert(kv_pair.k.clone(), kv_pair.v.clone());
        }
    }
    true
}

#[query(name = "logo")]
#[candid_method(query, rename = "logo")]
fn logo() -> Vec<u8> {
    unsafe { LOGO.clone() }
}

#[update(name = "setLogo")]
#[candid_method(update, rename = "setLogo")]
fn set_logo(logo: Vec<u8>) -> bool {
    _only_owner();
    unsafe { LOGO = logo }
    true
}

#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(holder: String) -> Nat {
    let token_holder_parse_result = holder.parse::<TokenHolder>();

    let balance = match token_holder_parse_result {
        Ok(token_holder) => _balance_of(&token_holder),
        _ => Nat::from(0),
    };
    ic_cdk::print(format!("get account balance is {}", balance));
    balance
}

fn _balance_of(holder: &TokenHolder) -> Nat {
    let balances = storage::get::<Balances>();
    match balances.get(holder) {
        Some(balance) => balance.clone(),
        None => Nat::from(0),
    }
}

#[query(name = "allowance")]
#[candid_method(query, rename = "allowance")]
fn allowance(owner: String, spender: String) -> Nat {
    let token_holder_owner_parse_result = owner.parse::<TokenHolder>();
    let token_holder_spender_parse_result = spender.parse::<TokenHolder>();

    let allowance: Nat = match token_holder_owner_parse_result {
        Ok(token_holder_owner) => match token_holder_spender_parse_result {
            Ok(token_holder_spender) => _allowance(&token_holder_owner, &token_holder_spender),
            _ => Nat::from(0),
        },
        _ => Nat::from(0),
    };

    ic_cdk::print(format!("get allowance is {}", allowance));
    allowance
}

fn _allowance(owner: &TokenHolder, spender: &TokenHolder) -> Nat {
    let allowances = storage::get::<Allowances>();
    match allowances.get(&owner) {
        Some(inner) => match inner.get(&spender) {
            Some(value) => value.clone(),
            None => Nat::from(0),
        },
        None => Nat::from(0),
    }
}

#[update(name = "approve")]
#[candid_method(update, rename = "approve")]
async fn approve(
    owner_sub_account: Option<Subaccount>,
    spender: String,
    value: Nat,
    call_data: Option<CallData>,
) -> ApproveResult {
    let caller = api::caller();
    let owner_holder = TokenHolder::new(caller, owner_sub_account);
    let spender_parse_result = spender.parse::<TokenHolder>();
    let approve_fee = _calc_approve_fee();

    if let Ok(spender_holder) = spender_parse_result {
        //charge approve, prevent gas ddos attacks
        match _charge_approve_fee(&spender_holder, approve_fee.clone()) {
            Ok(_) => {}
            Err(emsg) => return ApproveResult::Err(emsg),
        }

        let allowances_read = storage::get::<Allowances>();
        match allowances_read.get(&owner_holder) {
            Some(inner) => {
                let mut temp = inner.clone();
                let allowances = storage::get_mut::<Allowances>();
                if value == 0 {
                    temp.remove(&spender_holder);
                    if temp.len() > 0 {
                        allowances.insert(owner_holder.clone(), temp);
                    } else {
                        allowances.remove(&owner_holder);
                    }
                } else {
                    temp.insert(spender_holder.clone(), value.clone());
                    allowances.insert(owner_holder.clone(), temp);
                }
            }
            None => {
                if value.gt(&Nat::from(0)) {
                    let mut inner = HashMap::new();
                    inner.insert(spender_holder.clone(), value.clone());
                    let allowances = storage::get_mut::<Allowances>();
                    allowances.insert(owner_holder.clone(), inner);
                }
            }
        };
        let tx_index_new = _get_new_tx_index();
        let save_err_msg = _save_tx_record(TxRecord::Approve(
            tx_index_new.clone(),
            caller,
            owner_holder.clone(),
            spender_holder.clone(),
            value,
            approve_fee,
            api::time(),
        ))
        .await;
        let tx_id = encode_tx_id(api::id(), tx_index_new);

        let mut errors: Vec<String> = Vec::new();

        if save_err_msg.len() > 0 {
            errors.push(save_err_msg);
        }
        match call_data {
            Some(data) => {
                // execute call
                let execute_call_result = _execute_call(&spender_holder, data).await;

                match execute_call_result {
                    Err(emsg) => {
                        // approve succeed ,bu call failed
                        errors.push(emsg);
                        return ApproveResult::Ok(ApproveResponse {
                            txid: tx_id,
                            errors: Some(errors),
                        });
                    }
                    Ok(_) => {
                        return ApproveResult::Ok(ApproveResponse {
                            txid: tx_id,
                            errors: None,
                        })
                    }
                }
            }
            None => {
                return ApproveResult::Ok(ApproveResponse {
                    txid: tx_id,
                    errors: None,
                })
            }
        }
    } else {
        return ApproveResult::Err(MSG_INVALID_SPENDER.to_string());
    }
}

#[query(name = "getAllowancesByHolder")]
#[candid_method(query, rename = "getAllowancesByHolder")]
fn get_allowances_by_holder(holder: String) -> Vec<(TokenHolder, Nat)> {
    let allowances = storage::get::<Allowances>();
    match holder.parse::<TokenHolder>() {
        Ok(token_holder) => match allowances.get(&token_holder) {
            Some(allowance) => allowance.clone().into_iter().map(|x| (x.0, x.1)).collect(),
            None => Vec::new(),
        },
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
) -> TransferResult {
    let caller = api::caller();
    let spender = TokenHolder::new(caller, spender_sub_account);

    let from_parse_result = from.parse::<TokenHolder>();
    let to_parse_result = to.parse::<TokenHolder>();

    match from_parse_result {
        Ok(from_token_holder) => match to_parse_result {
            Ok(to_token_holder) => {
                let spender_allowance = _allowance(&from_token_holder, &spender);
                let fee = _calc_transfer_fee(value.clone());

                // check allowance
                if spender_allowance < value.clone() + fee.clone() {
                    return TransferResult::Err(MSG_ALLOWANCE_EXCEEDS.to_string());
                }
                let allowances_read = storage::get::<Allowances>();

                // update allowance
                match allowances_read.get(&from_token_holder) {
                    Some(inner) => {
                        let spender_allowance_new = spender_allowance - value.clone() - fee;
                        let mut temp = inner.clone();

                        if spender_allowance_new.gt(&Nat::from(0)) {
                            temp.insert(spender, spender_allowance_new);
                        } else {
                            temp.remove(&spender);
                        }
                        let allowances = storage::get_mut::<Allowances>();

                        if temp.len() > 0 {
                            allowances.insert(from_token_holder.clone(), temp);
                        } else {
                            allowances.remove(&from_token_holder);
                        }
                    }
                    None => {
                        //revert allowance
                        assert!(false);
                    }
                };
                // transfer
                _transfer(caller, from_token_holder, to_token_holder, value).await
            }
            _ => TransferResult::Err(MSG_INVALID_TO.to_string()),
        },
        _ => TransferResult::Err(MSG_INVALID_FROM.to_string()),
    }
}

#[update(name = "transfer")]
#[candid_method(update, rename = "transfer")]
async fn transfer(
    from_sub_account: Option<Subaccount>,
    to: String,
    value: Nat,
    call_data: Option<CallData>,
) -> TransferResult {
    let caller = api::caller();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let receiver_parse_result = to.parse::<TokenReceiver>();

    match receiver_parse_result {
        Ok(receiver) => {
            let mut errors: Vec<String> = Vec::new();
            match _transfer(caller, transfer_from, receiver.clone(), value).await {
                TransferResult::Ok(tx_res) => {
                    if let Some(inner_errors) = tx_res.error {
                        errors = [errors, inner_errors].concat();
                    }
                    if let Some(_call_data) = call_data {
                        // execute call
                        let execute_call_result = _execute_call(&receiver, _call_data).await;
                        if let Err(emsg) = execute_call_result {
                            errors.push(emsg);
                        };
                    }
                    if errors.len() > 0 {
                        TransferResult::Ok(TransferResponse {
                            txid: tx_res.txid,
                            error: Some(errors),
                        })
                    } else {
                        TransferResult::Ok(TransferResponse {
                            txid: tx_res.txid,
                            error: None,
                        })
                    }
                }
                TransferResult::Err(emsg) => return TransferResult::Err(emsg),
            }
        }
        _ => TransferResult::Err(MSG_INVALID_FROM.to_string()),
    }
}

async fn _transfer(
    caller: Principal,
    from: TokenHolder,
    to: TokenHolder,
    value: Nat,
) -> TransferResult {
    let fee = _calc_transfer_fee(value.clone());
    let from_balance = _balance_of(&from);

    if from_balance < value.clone() + fee.clone() {
        return TransferResult::Err(MSG_BALANCE_EXCEEDS.to_string());
    }

    // before transfer
    let before_sending_check_result = _on_token_sending(&from, &to, &value);

    if let Err(emsg) = before_sending_check_result {
        return TransferResult::Err(emsg);
    }

    let to_balance = _balance_of(&to);
    let balances = storage::get_mut::<Balances>();
    let from_balance_new = from_balance - value.clone() - fee.clone();

    if from_balance_new == 0 {
        balances.remove(&from);
    } else {
        balances.insert(from.clone(), from_balance_new);
    }
    balances.insert(to.clone(), to_balance + value.clone());
    _fee_settle(fee.clone());
    let tx_index_new = _get_new_tx_index();
    let save_err_msg = _save_tx_record(TxRecord::Transfer(
        tx_index_new.clone(),
        caller,
        from.clone(),
        to.clone(),
        value.clone(),
        fee,
        api::time(),
    ))
    .await;

    let mut errors: Vec<String> = Vec::new();

    if save_err_msg.len() > 0 {
        errors.push(save_err_msg)
    }

    // after transfer (notify)
    let after_token_send_notify_result = _on_token_received(&from, &to, &value).await;

    let tx_id = encode_tx_id(api::id(), tx_index_new);
    ic_cdk::print(format!("transfer tx id {}", tx_id));
    if let Err(emsg) = after_token_send_notify_result {
        errors.push(emsg);
        TransferResult::Ok(TransferResponse {
            txid: tx_id,
            error: Some(errors),
        })
    } else {
        TransferResult::Ok(TransferResponse {
            txid: tx_id,
            error: None,
        })
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(from_sub_account: Option<Subaccount>, value: Nat) -> BurnResult {
    let caller = api::caller();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let fee = _calc_transfer_fee(value.clone());

    if fee.gt(&value) {
        return BurnResult::Err(MSG_BURN_VALUE_TOO_SMALL.to_string());
    }

    let from_balance = _balance_of(&transfer_from);

    if from_balance < value {
        return BurnResult::Err(MSG_BURN_VALUE_EXCEEDS.to_string());
    }

    return _burn(caller, transfer_from, value).await;
}

async fn _burn(caller: Principal, from: TokenHolder, value: Nat) -> BurnResult {
    let from_balance = _balance_of(&from);

    let balances = storage::get_mut::<Balances>();

    let from_balance_new = from_balance - value.clone();

    if from_balance_new == 0 {
        balances.remove(&from);
    } else {
        balances.insert(from.clone(), from_balance_new);
    }

    let mut rw_total_supply = TOTAL_SUPPLY.write().unwrap();
    *rw_total_supply -= value.clone();
    let tx_index_new = _get_new_tx_index();
    let err_save_msg = _save_tx_record(TxRecord::Burn(
        tx_index_new.clone(),
        caller,
        from.clone(),
        value,
        api::time(),
    ))
    .await;

    let tx_id = encode_tx_id(api::id(), tx_index_new);
    BurnResult::Ok(BurnResponse {
        txid: tx_id,
        error: if err_save_msg.len() > 0 {
            Some(vec![err_save_msg])
        } else {
            None
        },
    })
}

#[update(name = "setFee")]
#[candid_method(update, rename = "setFee")]
fn set_fee(fee: Fee) -> bool {
    _only_owner();
    *FEE.write().unwrap() = fee;
    true
}

#[query(name = "setFeeTo")]
#[candid_method(update, rename = "setFeeTo")]
fn set_fee_to(fee_to: String) -> bool {
    _only_owner();
    let fee_to_parse_result = fee_to.parse::<TokenReceiver>();
    match fee_to_parse_result {
        Ok(holder) => _set_fee_to(holder),
        Err(_) => api::trap(MSG_INVALID_FEE_TO),
    }
}

fn _set_fee_to(holder: TokenHolder) -> bool {
    unsafe {
        FEE_TO = holder;
        true
    }
}

#[query(name = "getTokenInfo")]
#[candid_method(query, rename = "getTokenInfo")]
fn get_token_info() -> TokenInfo {
    let cycles = api::canister_balance();
    unsafe {
        TokenInfo {
            owner: OWNER,
            holders: Nat::from(storage::get::<Balances>().len()),
            allowance_size: Nat::from(storage::get::<Allowances>().len()),
            fee_to: FEE_TO.clone(),
            tx_count: (*TX_ID_CURSOR.read().unwrap()).clone(),
            cycles,
        }
    }
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}

#[pre_upgrade]
fn pre_upgrade() {
    let owner = unsafe { OWNER };
    let fee_to = unsafe { FEE_TO.clone() };
    let meta = get_meta_data();
    let logo = unsafe { LOGO.clone() };
    let tx_id_cursor = (*TX_ID_CURSOR.read().unwrap()).clone();

    let mut extend = Vec::new();
    let mut balances = Vec::new();
    let mut allowances = Vec::new();
    let mut storage_canister_ids = Vec::new();

    for (k, v) in storage::get_mut::<ExtendData>().iter() {
        extend.push((k.to_string(), v.to_string()));
    }
    for (k, v) in storage::get_mut::<Balances>().iter() {
        balances.push((k.clone(), v.clone()));
    }
    for (th, v) in storage::get_mut::<Allowances>().iter() {
        let mut allow_item = Vec::new();
        for (sp, val) in v.iter() {
            allow_item.push((sp.clone(), val.clone()));
        }
        allowances.push((th.clone(), allow_item));
    }
    for (k, v) in storage::get_mut::<StorageCanisterIds>().iter() {
        storage_canister_ids.push((k.clone(), *v));
    }
    let payload = TokenPayload {
        owner,
        fee_to,
        meta,
        extend,
        logo,
        balances,
        allowances,
        tx_id_cursor,
        storage_canister_ids,
    };
    storage::stable_save((payload,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    // There can only be one value in stable memory, currently. otherwise, lifetime error.
    // https://docs.rs/ic-cdk/0.3.0/ic_cdk/storage/fn.stable_restore.html
    let (payload,): (TokenPayload,) = storage::stable_restore().unwrap();
    unsafe {
        OWNER = payload.owner;
        FEE_TO = payload.fee_to;
        NAME = Box::leak(payload.meta.name.into_boxed_str());
        SYMBOL = Box::leak(payload.meta.symbol.into_boxed_str());
        DECIMALS = payload.meta.decimals;
        *TOTAL_SUPPLY.write().unwrap() = payload.meta.total_supply;
        *FEE.write().unwrap() = payload.meta.fee;
        *TX_ID_CURSOR.write().unwrap() = payload.tx_id_cursor;
        LOGO = payload.logo;
    }
    for (k, v) in payload.extend {
        storage::get_mut::<ExtendData>().insert(k, v);
    }
    for (k, v) in payload.balances {
        storage::get_mut::<Balances>().insert(k, v);
    }
    for (k, v) in payload.allowances {
        let mut inner = HashMap::new();
        for (ik, iv) in v {
            inner.insert(ik, iv);
        }
        storage::get_mut::<Allowances>().insert(k, inner);
    }
    for (k, v) in payload.storage_canister_ids {
        storage::get_mut::<StorageCanisterIds>().insert(k, v);
    }
}

// do something becore sending
fn _on_token_sending(
    #[warn(unused_variables)] _transfer_from: &TokenHolder,
    #[warn(unused_variables)] _receiver: &TokenReceiver,
    #[warn(unused_variables)] _value: &Nat,
) -> Result<(), String> {
    Ok(())
}

// call it after transfer, notify receiver with (from,value)
async fn _on_token_received(
    transfer_from: &TransferFrom,
    receiver: &TokenReceiver,
    _value: &Nat,
) -> Result<bool, String> {
    let get_did_method_name = "__get_candid_interface_tmp_hack";
    let on_token_received_method_name = "on_token_received";
    let on_token_received_method_sig = "on_token_received:(TransferFrom,nat)->(bool)query";

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

                    ic_cdk::print("notify executed!");

                    match _check_res {
                        Ok((is_notify_succeed,)) => {
                            if !is_notify_succeed {
                                return Err(MSG_NOTIFICATION_FAILED.to_string());
                            } else {
                                return Ok(true);
                            }
                        }
                        _ => return Err(MSG_NOTIFICATION_FAILED.to_string()),
                    }
                }
            }
            return Err(MSG_NOTIFICATION_FAILED.to_string());
        }
    }
    Ok(true)
}

async fn _execute_call(receiver: &TokenReceiver, _call_data: CallData) -> Result<bool, String> {
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
                        return Err(format!("DFT: call failed,details:{:?}", r.1));
                    }
                }
                Err(e) => return Err(format!("DFT: call failed,code:{:?},details:{:?}", e.0, e.1)),
            };
        }
    }
    Ok(true)
}

fn _calc_approve_fee() -> Nat {
    return FEE.read().unwrap().lowest.clone();
}

fn _calc_transfer_fee(value: Nat) -> Nat {
    let r_fee = FEE.read().unwrap();
    let div_by: Nat = BigUint::from(10u32).pow(FEE_RATE_DECIMALS as u32).into();
    let calc_fee: Nat = value.mul(r_fee.rate.clone()).div(div_by);
    cmp::max(r_fee.lowest.clone(), calc_fee)
}

fn _charge_approve_fee(payer: &TokenHolder, fee: Nat) -> Result<bool, String> {
    if fee == 0 {
        return Ok(true);
    }

    let balances = storage::get_mut::<Balances>();
    let payer_balance = _balance_of(&payer);
    if payer_balance < fee {
        return Err(MSG_INSUFFICIENT_BALANCE.to_string());
    }
    balances.insert(payer.clone(), payer_balance - fee.clone());
    _fee_settle(fee);
    Ok(true)
}

fn _fee_settle(fee: Nat) {
    if !fee.gt(&Nat::from(0)) {
        return;
    }
    let balances = storage::get_mut::<Balances>();
    unsafe {
        let fee_to_balance = _balance_of(&FEE_TO);
        balances.insert(FEE_TO.clone(), fee_to_balance + fee);
    }
}

async fn _save_tx_record(tx: TxRecord) -> String {
    let txs = storage::get_mut::<Txs>();
    txs.push(tx);

    let last_tx_index = _get_tx_index(&txs[0]);
    if txs.len() >= MAX_TXS_CACHE_IN_DFT {
        let storage_canister_id_res = _get_available_storage_id(&last_tx_index).await;

        match storage_canister_id_res {
            Ok(storage_canister_id) => {
                let should_save_txs = txs[0..MAX_TXS_CACHE_IN_DFT].to_vec();
                api::print(format!(
                    "should save tx length is {}",
                    should_save_txs.len()
                ));
                //save to auto-scaling storage
                match api::call::call(storage_canister_id, "batchAppend", (should_save_txs,)).await
                {
                    Ok((res,)) if res => {
                        api::print("append save res true");
                        let txs_after_call = storage::get_mut::<Txs>();
                        txs[0..MAX_TXS_CACHE_IN_DFT].iter().for_each(|_| {
                            txs_after_call.remove(0);
                        });
                    }
                    Err((_, emsg)) => {
                        api::print(format!(
                            "batchAppend: save to auto-scaling storage failed,{}  ",
                            emsg
                        ));
                    }
                    _ => {
                        api::print("append save res false?");
                    }
                }
            }
            Err(emsg) => {
                //Fallback: if create auto-scaling storage failed, do not remove it from dft cache storage.
                //Possible reasons for failure:
                //    1. Not enough cycles balance to create auto-scaling storage.
                //    2. Other unknown reason.
                api::print(
                    "save to auto-scaling storage failed, do not remove it from dft cache storage",
                );
                return emsg;
            }
        };
    }

    "".to_string()
}

fn _get_new_tx_index() -> Nat {
    let mut rw_tx_id_cursor = TX_ID_CURSOR.write().unwrap();
    *rw_tx_id_cursor += 1;

    rw_tx_id_cursor.clone()
}

fn _get_tx_index(tx: &TxRecord) -> Nat {
    match tx {
        TxRecord::Approve(ti, _, _, _, _, _, _) => ti.clone(),
        TxRecord::Transfer(ti, _, _, _, _, _, _) => ti.clone(),
        TxRecord::Burn(ti, _, _, _, _) => ti.clone(),
    }
}

async fn _get_available_storage_id(tx_index: &Nat) -> Result<Principal, String> {
    let mut max_key = Nat::from(0);
    let mut last_storage_id = Principal::anonymous();
    for (k, v) in storage::get::<StorageCanisterIds>().iter() {
        if k >= &max_key && last_storage_id != *v {
            max_key = k.clone();
            last_storage_id = v.clone();
        }
    }
    let mut is_necessary_create_new_storage_canister = last_storage_id == Principal::anonymous();

    // check storage remain size
    if !is_necessary_create_new_storage_canister {
        let req = CanisterIdRecord {
            canister_id: last_storage_id,
        };
        let status = get_canister_status(req).await;
        match status {
            Ok(res) => {
                ic_cdk::print(format!("memory_size is {}", res.memory_size));
                let min_storage_size_for_cache_txs =
                    Nat::from(MAX_TXS_CACHE_IN_DFT * std::mem::size_of::<TxRecord>());

                if (Nat::from(MAX_HEAP_MEMORY_SIZE) - res.memory_size)
                    .lt(&min_storage_size_for_cache_txs)
                {
                    is_necessary_create_new_storage_canister = true;
                } else {
                    return Ok(last_storage_id);
                }
            }
            Err(_) => {
                //api::trap(format!("get_canister_status failed {}", emsg).as_str());
                return Err(MSG_STORAGE_SCALING_FAILED.to_string());
            }
        };
    }

    if is_necessary_create_new_storage_canister {
        const STORAGE_WASM: &[u8] = std::include_bytes!(
            "../../../target/wasm32-unknown-unknown/release/dft_tx_storage_opt.wasm"
        );
        let dft_id = api::id();
        let create_args = CreateCanisterArgs {
            cycles: CYCLES_PER_TOKEN,
            settings: CanisterSettings {
                controllers: Some(vec![dft_id.clone()]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            },
        };
        api::print("creating token storage...");
        let create_result = create_canister(create_args).await;

        match create_result {
            Ok(cdr) => {
                api::print(format!(
                    "token new storage canister id : {} ,start index is {}",
                    cdr.canister_id.clone().to_string(),
                    tx_index.clone()
                ));

                let install_args = encode_args((dft_id.clone(), tx_index.clone()))
                    .expect("Failed to encode arguments.");

                match install_canister(&cdr.canister_id, STORAGE_WASM.to_vec(), install_args).await
                {
                    Ok(_) => {
                        storage::get_mut::<StorageCanisterIds>()
                            .insert(tx_index.clone(), cdr.canister_id);
                        return Ok(cdr.canister_id);
                    }
                    Err(emsg) => {
                        api::print(format!(
                            "install auto-scaling storage canister failed. details:{}",
                            emsg
                        ));
                        return Err(MSG_STORAGE_SCALING_FAILED.to_string());
                    }
                }
            }
            Err(emsg) => {
                api::print(format!("create new storage canister failed {}", emsg).as_str());
                return Err(MSG_STORAGE_SCALING_FAILED.to_string());
            }
        };
    } else {
        return Ok(last_storage_id);
    }
}

fn _only_owner() {
    unsafe {
        if OWNER != api::caller() {
            api::trap(MSG_ONLY_OWNER);
        }
    }
}
