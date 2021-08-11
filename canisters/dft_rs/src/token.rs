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
    KeyValuePair, MetaData, Subaccount, TokenHolder, TokenPayload, TokenReceiver, TransferFrom,
    TransferResult, TxRecord,
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

static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();
static mut NAME: &str = "";
static mut SYMBOL: &str = "";
static mut DECIMALS: u8 = 0;
static mut TOTAL_SUPPLY: u128 = 0;
static mut FEE: Fee = Fee { lowest: 0, rate: 0 };
static mut TOTAL_FEE: u128 = 0;
static mut TX_ID_CURSOR: u128 = 0;
// 256 * 256
static mut LOGO: Vec<u8> = Vec::new();
static mut STORAGE_CANISTER_ID: Principal = Principal::anonymous();
static mut FEE_CASHIER: TokenHolder = TokenHolder::Principal(Principal::anonymous());

#[init]
fn canister_init() {
    unsafe {
        OWNER = api::caller();
        FEE_CASHIER = TokenHolder::Principal(api::caller());
    }
}

#[update(name = "initialize")]
#[candid_method(update, rename = "initialize")]
async fn initialize(
    /*subaccount: Option<Subaccount>,
    logo: Vec<u8>,*/
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
) -> bool {
    _must_set_tx_storage();
    _only_owner();

    unsafe {
        if INITIALIZED {
            api::trap("initialized");
        }
        INITIALIZED = true;
        //LOGO = logo;
        NAME = Box::leak(name.into_boxed_str());
        SYMBOL = Box::leak(symbol.into_boxed_str());
        DECIMALS = decimals;
        TOTAL_SUPPLY = total_supply;
        let call_from = parse_to_token_holder(OWNER, None);
        let balances = storage::get_mut::<Balances>();
        balances.insert(call_from.clone(), TOTAL_SUPPLY);
    }
    true
}

#[query(name = "meta")]
#[candid_method(query, rename = "meta")]
fn get_meta_data() -> MetaData {
    _must_initialized();
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
    _must_initialized();
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
fn _update_extend_data(extend_data: Vec<KeyValuePair>) -> bool {
    _must_initialized();
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
fn _logo() -> Vec<u8> {
    unsafe { LOGO.clone() }
}

#[update(name = "updateLogo")]
#[candid_method(update, rename = "updateLogo")]
fn _update_logo(logo: Vec<u8>) -> bool {
    _must_initialized();
    _only_owner();
    unsafe { LOGO = logo }
    true
}

#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(holder: String) -> u128 {
    let token_holder_parse_result = holder.parse::<TokenHolder>();

    let balance = match token_holder_parse_result {
        Ok(token_holder) => _ibalance_of(&token_holder),
        _ => 0,
    };
    ic_cdk::print(format!("get account balance is {}", balance));
    balance
}

fn _ibalance_of(holder: &TokenHolder) -> u128 {
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
            Ok(token_holder_spender) => {
                _inner_allowance(&token_holder_owner, &token_holder_spender)
            }
            _ => 0u128,
        },
        _ => 0u128,
    };

    ic_cdk::print(format!("get allowance is {}", allowance));
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

#[update(name = "approve")]
#[candid_method(update, rename = "approve")]
async fn approve(
    owner_sub_account: Option<Subaccount>,
    spender: String,
    value: u128,
    call_data: Option<CallData>,
) -> ApproveResult {
    _must_initialized();
    _must_set_tx_storage();
    let owner = api::caller();
    let owner_holder = parse_to_token_holder(owner, owner_sub_account);
    let spender_parse_result = spender.parse::<TokenHolder>();
    let approve_fee = _calc_approve_fee();

    if let Ok(spender_holder) = spender_parse_result {
        //charge approve, prevent gas ddos attacks
        match _charge_approve_fee(&spender_holder, approve_fee) {
            Ok(_) => {}
            Err(emsg) => return ApproveResult::Err(emsg),
        }

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
                inner.insert(spender_holder.clone(), value - approve_fee);
                let allowances = storage::get_mut::<Allowances>();
                allowances.insert(owner_holder.clone(), inner);
            }
        };
        unsafe {
            _save_tx_record_to_graphql(TxRecord::Approve(
                owner.clone(),
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
    _must_initialized();
    _must_set_tx_storage();
    let spender_principal_id = api::caller();
    let spender = parse_to_token_holder(spender_principal_id, spender_sub_account);

    let from_parse_result = from.parse::<TokenHolder>();
    let to_parse_result = to.parse::<TokenHolder>();

    match from_parse_result {
        Ok(from_token_holder) => match to_parse_result {
            Ok(to_token_holder) => {
                let from_allowance = _inner_allowance(&from_token_holder, &spender);
                let fee = _calc_transfer_fee(value);

                // check allowance
                if from_allowance < value + fee {
                    return TransferResult::Err(
                        "DFT: transfer amount exceeds allowance".to_string(),
                    );
                }
                let allowances_read = storage::get::<Allowances>();

                // update allowance
                match allowances_read.get(&from_token_holder) {
                    Some(inner) => {
                        let mut temp = inner.clone();
                        temp.insert(spender, from_allowance - value - fee);
                        let allowances = storage::get_mut::<Allowances>();
                        allowances.insert(from_token_holder.clone(), temp);
                    }
                    None => {
                        //revert allowance
                        assert!(false);
                    }
                };
                // transfer
                _inner_transfer(
                    spender_principal_id,
                    from_token_holder,
                    to_token_holder,
                    value,
                )
                .await
            }
            _ => TransferResult::Err("DFT: invalid [to] format".to_string()),
        },
        _ => TransferResult::Err("DFT: invalid [from] format".to_string()),
    }
}

#[update(name = "transfer")]
#[candid_method(update, rename = "transfer")]
async fn _transfer(
    from_sub_account: Option<Subaccount>,
    to: String,
    value: u128,
    call_data: Option<CallData>,
) -> TransferResult {
    _must_initialized();
    _must_set_tx_storage();
    let from = api::caller();
    let transfer_from = parse_to_token_holder(from, from_sub_account);
    let receiver_parse_result = to.parse::<TokenReceiver>();

    match receiver_parse_result {
        Ok(receiver) => {
            let mut errors: Vec<String> = Vec::new();
            match _inner_transfer(from, transfer_from, receiver.clone(), value).await {
                TransferResult::Ok(txid, inner_errors_opt) => {
                    if let Some(inner_errors) = inner_errors_opt {
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
                        TransferResult::Ok(txid, Some(errors))
                    } else {
                        TransferResult::Ok(txid, None)
                    }
                }
                TransferResult::Err(emsg) => return TransferResult::Err(emsg),
            }
        }
        _ => TransferResult::Err("DFT: invalid [to] fromat".to_string()),
    }
}

async fn _inner_transfer(
    caller: Principal,
    from: TokenHolder,
    to: TokenHolder,
    value: u128,
) -> TransferResult {
    let fee = _calc_transfer_fee(value);
    let from_balance = _ibalance_of(&from);

    if from_balance < value + fee {
        return TransferResult::Err("DFT: transfer amount exceeds balance".to_string());
    }

    // before transfer
    let before_sending_check_result = _on_token_sending(&from, &to, &value);

    if let Err(emsg) = before_sending_check_result {
        return TransferResult::Err(emsg);
    }

    let to_balance = _ibalance_of(&to);
    let balances = storage::get_mut::<Balances>();

    balances.insert(from.clone(), from_balance - value - fee);
    balances.insert(to.clone(), to_balance + value);
    _fee_settle(fee);

    unsafe {
        let next_tx_id = _save_tx_record_to_graphql(TxRecord::Transfer(
            caller,
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
            TransferResult::Ok(next_tx_id, Some(errors))
        } else {
            TransferResult::Ok(next_tx_id, None)
        }
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn _burn(from_sub_account: Option<Subaccount>, value: u128) -> BurnResult {
    _must_initialized();
    _must_set_tx_storage();
    let from = api::caller();
    let transfer_from = parse_to_token_holder(from, from_sub_account);
    let fee = _calc_transfer_fee(value);

    if fee > value {
        return BurnResult::Err("DFT: burning value is too small".to_string());
    }

    let from_balance = _ibalance_of(&transfer_from);

    if from_balance < value {
        return BurnResult::Err("DFT: burn amount exceeds balance".to_string());
    }

    let balances = storage::get_mut::<Balances>();
    balances.insert(transfer_from.clone(), from_balance - value);
    unsafe {
        _save_tx_record_to_graphql(TxRecord::Burn(
            from.clone(),
            transfer_from.clone(),
            value,
            api::time(),
        ))
        .await;
    }
    BurnResult::Ok
}

#[query(name = "supportedInterface")]
#[candid_method(query, rename = "supportedInterface")]
fn supported_interface(interface: String) -> bool {
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

#[update(name = "setStorageCanisterID")]
#[candid_method(update, rename = "setStorageCanisterID")]
fn set_storage_canister_id(storage: Principal) -> bool {
    _only_owner();
    unsafe {
        STORAGE_CANISTER_ID = storage;
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

#[query(name = "setFeeCashier")]
#[candid_method(update, rename = "setFeeCashier")]
fn set_fee_cashier(holder: TokenHolder) -> bool {
    _only_owner();
    unsafe {
        FEE_CASHIER = holder;
        true
    }
}

#[query(name = "tokenGraphql")]
#[candid_method(query, rename = "tokenGraphql")]
fn _token_graphql() -> Principal {
    unsafe { STORAGE_CANISTER_ID }
}

candid::export_service!();

#[query(name = "__export_did_tmp")]
#[candid_method(query, rename = "__export_did_tmp")]
fn __export_did_tmp_() -> String {
    __export_service()
}

#[pre_upgrade]
fn pre_upgrade() {
    let initialized = unsafe { INITIALIZED };
    let owner = unsafe { OWNER };
    let fee_cashier = unsafe { FEE_CASHIER.clone() };
    let meta = get_meta_data();
    let logo = unsafe { LOGO.clone() };
    let total_fee = unsafe { TOTAL_FEE };
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
        initialized,
        owner,
        fee_cashier,
        meta,
        extend,
        logo,
        balances,
        allowances,
        total_fee,
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
        INITIALIZED = payload.initialized;
        OWNER = payload.owner;
        FEE_CASHIER = payload.fee_cashier;
        NAME = Box::leak(payload.meta.name.into_boxed_str());
        SYMBOL = Box::leak(payload.meta.symbol.into_boxed_str());
        DECIMALS = payload.meta.decimals;
        TOTAL_SUPPLY = payload.meta.total_supply;
        FEE = payload.meta.fee;
        TOTAL_FEE = payload.total_fee;
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
    let supported_interface_method_name = "supportedInterface";
    let on_token_received_method_name = "on_token_received";
    let on_token_received_method_sig = "on_token_received:(TransferFrom,nat128)->(bool)query";

    // check receiver
    if let TokenHolder::Principal(cid) = receiver {
        if utils::is_canister(cid) {
            let support_res: Result<(bool,), _> = api::call::call(
                *cid,
                supported_interface_method_name,
                (on_token_received_method_sig,),
            )
            .await;

            if let Ok((_support,)) = support_res {
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
                                return Err("DFT: notification failed".to_string());
                            }
                        }
                        _ => return Err("DFT: notification failed".to_string()),
                    }
                }
            }
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
                    return Ok(r.0);
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
        cmp::max(FEE.lowest, value * (FEE.rate as u128) / div_by)
    }
}

fn _charge_approve_fee(payer: &TokenHolder, fee: u128) -> Result<bool, String> {
    if fee == 0 {
        return Ok(true);
    }

    let balances = storage::get_mut::<Balances>();
    let payer_balance = _ibalance_of(&payer);
    if payer_balance < fee {
        return Err("DFT: insufficient balance,failed to charge approval fee".to_string());
    }
    balances.insert(payer.clone(), payer_balance - fee);
    _fee_settle(fee);
    Ok(true)
}

fn _fee_settle(fee: u128) {
    if fee > 0 {
        let balances = storage::get_mut::<Balances>();
        unsafe {
            let fee_to_balance = _ibalance_of(&FEE_CASHIER);
            balances.insert(FEE_CASHIER.clone(), fee_to_balance + fee);
            TOTAL_FEE += fee;
        }
    }
}

async fn _save_tx_record_to_graphql(tx: TxRecord) -> u128 {
    _must_set_tx_storage();
    unsafe {
        TX_ID_CURSOR += 1;
        let type_str: &str;
        let call_str: String;
        let from_str: String;
        let to_str: String;
        let value_str: String;
        let fee_str: String;
        let timestamp_str: String;
        match tx {
            TxRecord::Approve(caller, owner, spender, value, fee, t) => {
                type_str = TX_TYPES_APPROVE;
                call_str = caller.to_string();
                from_str = owner.to_string();
                to_str = spender.to_string();
                value_str = value.to_string();
                fee_str = fee.to_string();
                timestamp_str = t.to_string();
            }
            TxRecord::Transfer(caller, from, to, value, fee, t) => {
                type_str = TX_TYPES_TRANSFER;
                call_str = caller.to_string();
                from_str = from.to_string();
                to_str = to.to_string();
                value_str = value.to_string();
                fee_str = fee.to_string();
                timestamp_str = t.to_string();
            }
            TxRecord::Burn(caller, from, value, t) => {
                type_str = TX_TYPES_BURN;
                call_str = caller.to_string();
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
                                caller:"{2}",from:"{3}",
                                to:"{4}",value:"{5}",
                                fee:"{6}",timestamp:"{7}",
                                }}) 
                                {{ id }} 
                               }}"#,
            TX_ID_CURSOR, type_str, call_str, from_str, to_str, value_str, fee_str, timestamp_str
        );
        //call storage canister
        let _support_res: Result<(String,), _> = api::call::call(
            STORAGE_CANISTER_ID,
            "graphql_mutation",
            (muation.to_string(), vals),
        )
        .await;
        ic_cdk::print(format!("muation is :{}", muation.to_string()));
        match _support_res {
            Ok(res) => ic_cdk::print(format!("graph write succeed :{}", res.0)),
            Err((_, msg)) => ic_cdk::print(format!("graph write error :{}", msg)),
        };
        TX_ID_CURSOR
    }
}

fn _only_owner() {
    unsafe {
        if OWNER != api::caller() {
            api::trap("caller is not the owner");
        }
    }
}
fn _must_initialized() {
    unsafe {
        if !INITIALIZED {
            api::trap("uninitialized");
        }
    }
}

fn _must_set_tx_storage() {
    unsafe {
        if STORAGE_CANISTER_ID == Principal::anonymous() {
            api::trap("no storage canister");
        }
    }
}
