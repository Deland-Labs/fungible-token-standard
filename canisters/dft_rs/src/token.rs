/**
 * Module     : token.rs
 * Copyright  : 2021 Deland-Labs Team
 * License    : Apache 2.0 with LLVM Exception
 * Maintainer : Deland Team (https://deland.one)
 * Stability  : Experimental
 */
use crate::extends;
use crate::types::{
    AccountIdentifier, Allowances, ApproveResult, Balances, BurnResult, CallData, ExtendData, Fee,
    KeyValuePair, MetaData, StatisticsInfo, Subaccount, TokenHolder, TokenPayload, TokenReceiver,
    TransferFrom, TransferResponse, TransferResult, TxRecord,
};
use crate::utils;
use candid::{candid_method, decode_args, IDLProg};
use ic_cdk::{api, export::Principal, storage};
use ic_cdk_macros::*;
use std::cmp;
use std::collections::HashMap;
use std::string::String;

// transferFee = amount * rate / 10.pow(FEE_RATE_DECIMALS)
const FEE_RATE_DECIMALS: u8 = 8u8;
const TX_TYPES_APPROVE: &str = "approve";
const TX_TYPES_TRANSFER: &str = "transfer";
const TX_TYPES_BURN: &str = "burn";
// const TX_TYPES_MINT: &str = "mint";

const MSG_ONLY_OWNER: &str = "DFT: caller is not the owner";
const MSG_INVALID_SPENDER: &str = "DFT: invalid spender";
const MSG_INVALID_FROM: &str = "DFT: invalid format [from]";
const MSG_INVALID_TO: &str = "DFT: invalid format [to]";
const MSG_INVALID_FEE_TO: &str = "DFT: invalid format [feeTo]";
const MSG_FAILED_TO_CHARGE_FEE: &str = "DFT: Failed to charge fee - insufficient balance";
const MSG_ALLOWANCE_EXCEEDS: &str = "DFT: transfer amount exceeds allowance";
const MSG_BALANCE_EXCEEDS: &str = "DFT: transfer amount exceeds balance";
const MSG_BURN_VALUE_TOO_SMALL: &str = "DFT: burning value is too small";
const MSG_BURN_VALUE_EXCEEDS: &str = "DFT: burning value exceeds balance";
const MSG_NOTIFICATION_FAILED: &str = "DFT: notification failed";

static mut OWNER: Principal = Principal::anonymous();
static mut NAME: &str = "";
static mut SYMBOL: &str = "";
static mut DECIMALS: u8 = 0;
static mut TOTAL_SUPPLY: u128 = 0;
static mut FEE: Fee = Fee { lowest: 0, rate: 0 };
static mut TX_ID_CURSOR: u128 = 0;
// 256 * 256
static mut LOGO: Vec<u8> = Vec::new();
static mut STORAGE_CANISTER_ID: Principal = Principal::anonymous();
static mut FEE_TO: TokenHolder = TokenHolder::Principal(Principal::anonymous());

#[init]
fn canister_init(
    sub_account: Option<Subaccount>,
    logo: Option<Vec<u8>>,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
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
        NAME = Box::leak(name.into_boxed_str());
        SYMBOL = Box::leak(symbol.into_boxed_str());
        DECIMALS = decimals;
        TOTAL_SUPPLY = total_supply;
        FEE = fee;
        let call_from = parse_to_token_holder(caller, sub_account);
        FEE_TO = call_from.clone();
        ic_cdk::print(format!("caller is {}", caller.to_text()));
        match call_from {
            TokenHolder::Account(a) => ic_cdk::print(format!("init : account is {}", a.to_hex())),
            TokenHolder::Principal(p) => {
                ic_cdk::print(format!("init : account is {}", p.to_text()))
            }
        };
        let balances = storage::get_mut::<Balances>();
        balances.insert(call_from.clone(), TOTAL_SUPPLY);
    }
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
fn get_total_supply() -> u128 {
    unsafe { TOTAL_SUPPLY }
}

#[query(name = "fee")]
#[candid_method(query, rename = "fee")]
fn get_fee_setting() -> Fee {
    unsafe { FEE.clone() }
}

#[query(name = "meta")]
#[candid_method(query, rename = "meta")]
fn get_meta_data() -> MetaData {
    unsafe {
        let meta = MetaData {
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            decimals: DECIMALS,
            total_supply: TOTAL_SUPPLY,
            fee: FEE.clone(),
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

#[update(name = "updateExtend")]
#[candid_method(update, rename = "updateExtend")]
fn update_extend_data(extend_data: Vec<KeyValuePair>) -> bool {
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

#[update(name = "updateLogo")]
#[candid_method(update, rename = "updateLogo")]
fn update_logo(logo: Vec<u8>) -> bool {
    _only_owner();
    unsafe { LOGO = logo }
    true
}

#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(holder: String) -> u128 {
    let token_holder_parse_result = holder.parse::<TokenHolder>();

    let balance = match token_holder_parse_result {
        Ok(token_holder) => _balance_of(&token_holder),
        _ => 0,
    };
    ic_cdk::print(format!("get account balance is {}", balance));
    balance
}

fn _balance_of(holder: &TokenHolder) -> u128 {
    let balances = storage::get::<Balances>();
    match balances.get(holder) {
        Some(balance) => *balance,
        None => 0,
    }
}

#[query(name = "allowance")]
#[candid_method(query, rename = "allowance")]
fn allowance(owner: String, spender: String) -> u128 {
    let token_holder_owner_parse_result = owner.parse::<TokenHolder>();
    let token_holder_spender_parse_result = spender.parse::<TokenHolder>();

    let allowance: u128 = match token_holder_owner_parse_result {
        Ok(token_holder_owner) => match token_holder_spender_parse_result {
            Ok(token_holder_spender) => _allowance(&token_holder_owner, &token_holder_spender),
            _ => 0u128,
        },
        _ => 0u128,
    };

    ic_cdk::print(format!("get allowance is {}", allowance));
    allowance
}

fn _allowance(owner: &TokenHolder, spender: &TokenHolder) -> u128 {
    let allowances = storage::get::<Allowances>();
    match allowances.get(&owner) {
        Some(inner) => match inner.get(&spender) {
            Some(value) => *value,
            None => 0u128,
        },
        None => 0u128,
    }
}

#[update(name = "approve")]
#[candid_method(update, rename = "approve")]
async fn approve(
    owner_sub_account: Option<Subaccount>,
    spender: String,
    value: u128,
    call_data: Option<CallData>,
) -> ApproveResult {
    let owner = api::caller();
    let owner_holder = parse_to_token_holder(owner, owner_sub_account);
    let spender_parse_result = spender.parse::<TokenHolder>();
    let approve_fee = _calc_approve_fee();

    match spender_parse_result {
        Ok(spender_holder) => {
            //charge approve, prevent gas ddos attacks
            match _charge_approve_fee(&spender_holder, approve_fee) {
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
                        temp.insert(spender_holder.clone(), value);
                        allowances.insert(owner_holder.clone(), temp);
                    }
                }
                None => {
                    if value > 0 {
                        let mut inner = HashMap::new();
                        inner.insert(spender_holder.clone(), value);
                        let allowances = storage::get_mut::<Allowances>();
                        allowances.insert(owner_holder.clone(), inner);
                    }
                }
            };
            unsafe {
                _save_tx_record_to_graphql(TxRecord::Approve(
                    owner_holder.clone(),
                    spender_holder.clone(),
                    value,
                    approve_fee,
                    api::time(),
                ))
                .await;
            }

            if let Some(_call_data) = call_data {
                // execute call
                let execute_call_result = _execute_call(&spender_holder, _call_data).await;

                if let Err(emsg) = execute_call_result {
                    // approve succeed ,bu call failed
                    return ApproveResult::Ok(Some(emsg));
                };
            }
        }
        Err(_) => return ApproveResult::Err(MSG_INVALID_SPENDER.to_string()),
    }

    ApproveResult::Ok(None)
}

#[update(name = "transferFrom")]
#[candid_method(update, rename = "transferFrom")]
async fn transfer_from(
    spender_sub_account: Option<Subaccount>,
    from: String,
    to: String,
    value: u128,
) -> TransferResult {
    let spender_principal_id = api::caller();
    let spender = parse_to_token_holder(spender_principal_id, spender_sub_account);

    let from_parse_result = from.parse::<TokenHolder>();
    let to_parse_result = to.parse::<TokenHolder>();

    match from_parse_result {
        Ok(from_token_holder) => match to_parse_result {
            Ok(to_token_holder) => {
                let spender_allowance = _allowance(&from_token_holder, &spender);
                let fee = _calc_transfer_fee(value);

                // check allowance
                if spender_allowance < value + fee {
                    return TransferResult::Err(MSG_ALLOWANCE_EXCEEDS.to_string());
                }
                let allowances_read = storage::get::<Allowances>();

                // update allowance
                match allowances_read.get(&from_token_holder) {
                    Some(inner) => {
                        let spender_allowance_new = spender_allowance - value - fee;
                        let mut temp = inner.clone();

                        if spender_allowance_new > 0 {
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
                _transfer(from_token_holder, to_token_holder, value).await
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
    value: u128,
    call_data: Option<CallData>,
) -> TransferResult {
    let from = api::caller();
    let transfer_from = parse_to_token_holder(from, from_sub_account);
    let receiver_parse_result = to.parse::<TokenReceiver>();

    match receiver_parse_result {
        Ok(receiver) => {
            let mut errors: Vec<String> = Vec::new();
            match _transfer(transfer_from, receiver.clone(), value).await {
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

async fn _transfer(from: TokenHolder, to: TokenHolder, value: u128) -> TransferResult {
    let fee = _calc_transfer_fee(value);
    let from_balance = _balance_of(&from);

    if from_balance < value + fee {
        return TransferResult::Err(MSG_BALANCE_EXCEEDS.to_string());
    }

    // before transfer
    let before_sending_check_result = _on_token_sending(&from, &to, &value);

    if let Err(emsg) = before_sending_check_result {
        return TransferResult::Err(emsg);
    }

    let to_balance = _balance_of(&to);
    let balances = storage::get_mut::<Balances>();
    let from_balance_new = from_balance - value - fee;

    if from_balance_new == 0 {
        balances.remove(&from);
    } else {
        balances.insert(from.clone(), from_balance_new);
    }
    balances.insert(to.clone(), to_balance + value);
    _fee_settle(fee);

    unsafe {
        let next_tx_id = _save_tx_record_to_graphql(TxRecord::Transfer(
            from.clone(),
            to.clone(),
            value,
            fee,
            api::time(),
        ))
        .await;

        let mut errors: Vec<String> = Vec::new();

        // after transfer (notify)
        let after_token_send_notify_result = _on_token_received(&from, &to, &value).await;

        if let Err(emsg) = after_token_send_notify_result {
            errors.push(emsg);
            TransferResult::Ok(TransferResponse {
                txid: next_tx_id,
                error: Some(errors),
            })
        } else {
            TransferResult::Ok(TransferResponse {
                txid: next_tx_id,
                error: None,
            })
        }
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(from_sub_account: Option<Subaccount>, value: u128) -> BurnResult {
    let from = api::caller();
    let transfer_from = parse_to_token_holder(from, from_sub_account);
    let fee = _calc_transfer_fee(value);

    if fee > value {
        return BurnResult::Err(MSG_BURN_VALUE_TOO_SMALL.to_string());
    }

    let from_balance = _balance_of(&transfer_from);

    if from_balance < value {
        return BurnResult::Err(MSG_BURN_VALUE_EXCEEDS.to_string());
    }

    return _burn(transfer_from, value).await;
}

async fn _burn(from: TokenHolder, value: u128) -> BurnResult {
    let from_balance = _balance_of(&from);

    let balances = storage::get_mut::<Balances>();

    let from_balance_new = from_balance - value;

    if from_balance_new == 0 {
        balances.remove(&from);
    } else {
        balances.insert(from.clone(), from_balance_new);
    }
    unsafe {
        TOTAL_SUPPLY -= value;
        _save_tx_record_to_graphql(TxRecord::Burn(from.clone(), value, api::time())).await;
    }
    BurnResult::Ok
}

fn supported_interface(did: String, interface_sig: String) -> bool {
    let verify_service_desc = format!("service:{{ {0};}}", interface_sig);
    let verify_ast_result = verify_service_desc.parse::<IDLProg>();

    match verify_ast_result {
        Ok(verify_ast) => {
            let verify_pretty: String = candid::parser::types::to_pretty(&verify_ast, 80);
            let verify_pretty_sub: String =
                verify_pretty.replace("service : { ", "").replace(" }", "");

            let origin_did = did;
            let origin_ast: IDLProg = origin_did.parse().unwrap();
            let origin_pretty: String = candid::parser::types::to_pretty(&origin_ast, 80);
            origin_pretty.contains(&verify_pretty_sub)
        }
        _ => false,
    }
}

#[update(name = "setStorageCanisterID")]
#[candid_method(update, rename = "setStorageCanisterID")]
fn set_storage_canister_id(storage_canister_id_opt: Option<Principal>) -> bool {
    _only_owner();
    unsafe {
        match storage_canister_id_opt {
            Some(id) => STORAGE_CANISTER_ID = id,
            None => STORAGE_CANISTER_ID = Principal::anonymous(),
        }
        true
    }
}

#[update(name = "setFee")]
#[candid_method(update, rename = "setFee")]
fn set_fee(fee: Fee) -> bool {
    _only_owner();
    unsafe {
        FEE = fee;
        true
    }
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

#[query(name = "tokenGraphql")]
#[candid_method(query, rename = "tokenGraphql")]
fn _token_graphql() -> Option<Principal> {
    unsafe { Some(STORAGE_CANISTER_ID) }
}

#[query(name = "getStatistics")]
#[candid_method(query, rename = "getStatistics")]
fn get_statistics() -> StatisticsInfo {
    unsafe {
        StatisticsInfo {
            holders: storage::get_mut::<Balances>().len() as u128,
            transfers: TX_ID_CURSOR,
        }
    }
}

// query cycles balance
#[query(name = "cyclesBalance")]
#[candid_method(query, rename = "cyclesBalance")]
fn cycles_balance() -> u128 {
    api::canister_balance().into()
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
    let tx_id_cursor = unsafe { TX_ID_CURSOR };
    let storage_canister_id = unsafe { STORAGE_CANISTER_ID };

    let mut extend = Vec::new();
    let mut balances = Vec::new();
    let mut allowances = Vec::new();

    for (k, v) in storage::get_mut::<ExtendData>().iter() {
        extend.push((k.to_string(), v.to_string()));
    }
    for (k, v) in storage::get_mut::<Balances>().iter() {
        balances.push((k.clone(), *v));
    }
    for (th, v) in storage::get_mut::<Allowances>().iter() {
        let mut allow_item = Vec::new();
        for (sp, val) in v.iter() {
            allow_item.push((sp.clone(), *val));
        }
        allowances.push((th.clone(), allow_item));
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
        storage_canister_id,
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
        TOTAL_SUPPLY = payload.meta.total_supply;
        FEE = payload.meta.fee;
        TX_ID_CURSOR = payload.tx_id_cursor;
        LOGO = payload.logo;
        STORAGE_CANISTER_ID = payload.storage_canister_id;
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
}

fn parse_to_token_holder(from: Principal, from_sub_account: Option<Subaccount>) -> TransferFrom {
    match from_sub_account {
        Some(_) => {
            let account_identity = AccountIdentifier::new(from, from_sub_account);
            TransferFrom::Account(account_identity)
        }
        _ => TransferFrom::Principal(from),
    }
}

// do something becore sending
fn _on_token_sending(
    #[warn(unused_variables)] _transfer_from: &TokenHolder,
    #[warn(unused_variables)] _receiver: &TokenReceiver,
    #[warn(unused_variables)] _value: &u128,
) -> Result<(), String> {
    Ok(())
}

// call it after transfer, notify receiver with (from,value)
async fn _on_token_received(
    transfer_from: &TransferFrom,
    receiver: &TokenReceiver,
    _value: &u128,
) -> Result<bool, String> {
    let get_did_method_name = "__get_candid_interface_tmp_hack";
    let on_token_received_method_name = "on_token_received";
    let on_token_received_method_sig = "on_token_received:(TransferFrom,nat)->(bool)query";

    // check receiver
    if let TokenHolder::Principal(cid) = receiver {
        if utils::is_canister(cid) {
            let did_res: Result<(String,), _> =
                api::call::call(*cid, get_did_method_name, ()).await;

            if let Ok((did,)) = did_res {
                let _support = supported_interface(did, on_token_received_method_sig.to_string());
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
        if utils::is_canister(cid) {
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

fn _calc_approve_fee() -> u128 {
    unsafe {
        return FEE.lowest;
    }
}

fn _calc_transfer_fee(value: u128) -> u128 {
    unsafe {
        let div_by = (10 as u128).pow(FEE_RATE_DECIMALS as u32);
        cmp::max(FEE.lowest, value * FEE.rate / div_by)
    }
}

fn _charge_approve_fee(payer: &TokenHolder, fee: u128) -> Result<bool, String> {
    if fee == 0 {
        return Ok(true);
    }

    let balances = storage::get_mut::<Balances>();
    let payer_balance = _balance_of(&payer);
    if payer_balance < fee {
        return Err(MSG_FAILED_TO_CHARGE_FEE.to_string());
    }
    balances.insert(payer.clone(), payer_balance - fee);
    _fee_settle(fee);
    Ok(true)
}

fn _fee_settle(fee: u128) {
    if fee > 0 {
        let balances = storage::get_mut::<Balances>();
        unsafe {
            let fee_to_balance = _balance_of(&FEE_TO);
            balances.insert(FEE_TO.clone(), fee_to_balance + fee);
        }
    }
}

async fn _save_tx_record_to_graphql(tx: TxRecord) -> u128 {
    unsafe {
        TX_ID_CURSOR += 1;

        if STORAGE_CANISTER_ID == Principal::anonymous() {
            return TX_ID_CURSOR;
        }
        let type_str: &str;
        let from_str: String;
        let to_str: String;
        let value_str: String;
        let fee_str: String;
        let timestamp_str: String;
        match tx {
            TxRecord::Approve(owner, spender, value, fee, t) => {
                type_str = TX_TYPES_APPROVE;
                from_str = owner.to_string();
                to_str = spender.to_string();
                value_str = value.to_string();
                fee_str = fee.to_string();
                timestamp_str = t.to_string();
            }
            TxRecord::Transfer(from, to, value, fee, t) => {
                type_str = TX_TYPES_TRANSFER;
                from_str = from.to_string();
                to_str = to.to_string();
                value_str = value.to_string();
                fee_str = fee.to_string();
                timestamp_str = t.to_string();
            }
            TxRecord::Burn(from, value, t) => {
                type_str = TX_TYPES_BURN;
                from_str = from.to_string();
                to_str = "".to_string();
                value_str = value.to_string();
                fee_str = "0".to_string();
                timestamp_str = t.to_string();
            }
        }

        let vals = "{}".to_string();
        let muation = format!(
            r#"mutation {{ 
                            createTx(input: {{ 
                                txid:  "{0}",txtype:"{1}",
                                from:"{2}",to:"{3}",value:"{4}",
                                fee:"{5}",timestamp:"{6}",
                                }}) 
                                {{ id }} 
                               }}"#,
            TX_ID_CURSOR, type_str, from_str, to_str, value_str, fee_str, timestamp_str
        );
        //call storage canister
        let _support_res: Result<(String,), _> = api::call::call(
            STORAGE_CANISTER_ID,
            "graphql_mutation",
            (muation.to_string(), vals),
        )
        .await;
        ic_cdk::print(format!("muation is :{}", muation.to_string()));
        // match _support_res {
        //     Ok(res) => ic_cdk::print(format!("graph write succeed :{}", res.0)),
        //     Err((_, msg)) => ic_cdk::print(format!("graph write error :{}", msg)),
        // };
        TX_ID_CURSOR
    }
}

fn _only_owner() {
    unsafe {
        if OWNER != api::caller() {
            api::trap(MSG_ONLY_OWNER);
        }
    }
}
