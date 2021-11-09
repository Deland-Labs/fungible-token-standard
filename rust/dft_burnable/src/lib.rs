#[macro_use]
extern crate lazy_static;
extern crate dft_types;
extern crate dft_utils;
mod token;

use candid::{candid_method, decode_args, encode_args};
use dft_standard::{
    ic_management::*,
    token::{TokenBasic, TokenStandard},
};
use dft_types::{message::*, *};
use dft_utils::*;
use ic_cdk::{
    api,
    export::{candid::Nat, Principal},
    storage,
};
use ic_cdk_macros::*;
use std::{collections::HashMap, string::String, sync::RwLock};
use token::BurnableExtension;

// transferFee = amount * rate / 10.pow(FEE_RATE_DECIMALS)
const MAX_TXS_CACHE_IN_DFT: usize = 1000;
const MAX_HEAP_MEMORY_SIZE: u32 = 4294967295u32; // 4G
const CYCLES_PER_TOKEN: u64 = 2000_000_000_000; // 2T

lazy_static! {
    pub static ref TOKEN: RwLock<TokenBasic> = RwLock::new(TokenBasic::default());
}

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

    let mut token = TOKEN.write().unwrap();
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
}

#[update(name = "owner")]
#[candid_method(update, rename = "owner")]
fn owner() -> Principal {
    TOKEN.read().unwrap().owner()
}

#[update(name = "setOwner")]
#[candid_method(update, rename = "setOwner")]
fn set_owner(owner: Principal) -> Result<bool, String> {
    TOKEN.write().unwrap().set_owner(&api::caller(), owner)
}

#[query(name = "name")]
#[candid_method(query, rename = "name")]
fn get_name() -> String {
    TOKEN.read().unwrap().name()
}

#[query(name = "symbol")]
#[candid_method(query, rename = "symbol")]
fn get_symbol() -> String {
    TOKEN.read().unwrap().symbol()
}

#[query(name = "decimals")]
#[candid_method(query, rename = "decimals")]
fn get_decimals() -> u8 {
    TOKEN.read().unwrap().decimals()
}

#[query(name = "totalSupply")]
#[candid_method(query, rename = "totalSupply")]
fn get_total_supply() -> Nat {
    TOKEN.read().unwrap().total_supply()
}

#[query(name = "fee")]
#[candid_method(query, rename = "fee")]
fn get_fee_setting() -> Fee {
    TOKEN.read().unwrap().fee()
}

#[query(name = "meta")]
#[candid_method(query, rename = "meta")]
fn get_meta_data() -> Metadata {
    TOKEN.read().unwrap().metadata()
}

#[query(name = "desc")]
#[candid_method(query, rename = "desc")]
fn get_desc_info() -> Vec<(String, String)> {
    TOKEN
        .read()
        .unwrap()
        .desc()
        .into_iter()
        .map(|f| f)
        .collect()
}

#[update(name = "setDesc")]
#[candid_method(update, rename = "setDesc")]
fn set_desc_info(extend_data: Vec<(String, String)>) -> Result<bool, String> {
    // convert exntend data to hashmap
    let mut extend_info = HashMap::new();
    for (key, value) in extend_data {
        extend_info.insert(key, value);
    }
    TOKEN.write().unwrap().set_desc(&api::caller(), extend_info)
}

#[query(name = "logo")]
#[candid_method(query, rename = "logo")]
fn logo() -> Vec<u8> {
    TOKEN.read().unwrap().logo()
}

#[update(name = "setLogo")]
#[candid_method(update, rename = "setLogo")]
fn set_logo(logo: Vec<u8>) -> Result<bool, String> {
    TOKEN.write().unwrap().set_logo(&api::caller(), logo)
}

#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(holder: String) -> Nat {
    let token_holder_parse_result = holder.parse::<TokenHolder>();
    match token_holder_parse_result {
        Ok(token_holder) => TOKEN.read().unwrap().balance_of(&token_holder),
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
            return TOKEN
                .read()
                .unwrap()
                .allowance(&token_holder_owner, &token_holder_spender);
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
) -> TransactionResult {
    let caller = api::caller();
    let owner_holder = TokenHolder::new(caller.clone(), owner_sub_account);
    match spender.parse::<TokenHolder>() {
        Ok(spender_holder) => {
            let tx_index = TOKEN.write().unwrap().approve(
                &caller,
                &owner_holder,
                &spender_holder,
                value,
                api::time(),
            )?;
            let tx_id = encode_tx_id(api::id(), tx_index);

            let mut errors: Vec<String> = vec![];
            match exec_auto_scaling_strategy().await {
                Ok(_) => (),
                Err(e) => {
                    errors.push(e.to_string());
                }
            }
            if let Some(data) = call_data {
                // execute call
                let execute_call_result = execute_call(&spender_holder, data).await;
                if let Err(emsg) = execute_call_result {
                    // approve succeed ,bu call failed
                    errors.push(emsg.to_string());
                }
            };
            TransactionResult::Ok(TransactionResponse {
                txid: tx_id,
                error: if errors.len() > 0 { Some(errors) } else { None },
            })
        }
        Err(_) => TransactionResult::Err(MSG_INVALID_SPENDER.to_string()),
    }
}

#[query(name = "allowancesOf")]
#[candid_method(query, rename = "allowancesOf")]
fn allowances_of_holder(holder: String) -> Vec<(TokenHolder, Nat)> {
    match holder.parse::<TokenHolder>() {
        Ok(token_holder) => TOKEN.read().unwrap().allowances_of(&token_holder),
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
) -> TransactionResult {
    let caller = api::caller();
    let now = api::time();
    let spender = TokenHolder::new(caller, spender_sub_account);

    match from.parse::<TokenHolder>() {
        Ok(from_token_holder) => match to.parse::<TokenHolder>() {
            Ok(to_token_holder) => {
                // exec before-transfer check
                before_token_sending(&from_token_holder, &to_token_holder, &value)?;
                let tx_index = TOKEN.write().unwrap().transfer_from(
                    &caller,
                    &from_token_holder,
                    &spender,
                    &to_token_holder,
                    value.clone(),
                    now,
                )?;
                let mut errors: Vec<String> = vec![];
                // exec auto scaling strategy
                match exec_auto_scaling_strategy().await {
                    Err(e) => errors.push(e.to_string()),
                    _ => {}
                };
                // exec after-transfer notify
                match on_token_received(&from_token_holder, &to_token_holder, &value).await {
                    Err(e) => errors.push(e.to_string()),
                    _ => (),
                };
                TransactionResult::Ok(TransactionResponse {
                    txid: encode_tx_id(api::id(), tx_index),
                    error: if errors.len() > 0 { Some(errors) } else { None },
                })
            }
            _ => TransactionResult::Err(MSG_INVALID_TO.to_string()),
        },
        _ => TransactionResult::Err(MSG_INVALID_FROM.to_string()),
    }
}

#[update(name = "transfer")]
#[candid_method(update, rename = "transfer")]
async fn transfer(
    from_sub_account: Option<Subaccount>,
    to: String,
    value: Nat,
    call_data: Option<CallData>,
) -> TransactionResult {
    let caller = api::caller();
    let now = api::time();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let receiver_parse_result = to.parse::<TokenReceiver>();

    match receiver_parse_result {
        Ok(receiver) => {
            //exec before-transfer check
            before_token_sending(&transfer_from, &receiver, &value)?;
            let mut errors: Vec<String> = Vec::new();
            //transfer token
            let tx_index = TOKEN.write().unwrap().transfer(
                &caller,
                &transfer_from,
                &receiver,
                value.clone(),
                now,
            )?;

            //exec auto-scaling storage strategy
            match exec_auto_scaling_strategy().await {
                Ok(_) => (),
                Err(e) => {
                    errors.push(e.to_string());
                }
            };
            // exec after-transfer notify
            match on_token_received(&transfer_from, &receiver, &value).await {
                Err(e) => errors.push(e.to_string()),
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
            return TransactionResult::Ok(TransactionResponse {
                txid: encode_tx_id(api::id(), tx_index),
                error: if errors.len() > 0 { Some(errors) } else { None },
            });
        }
        _ => TransactionResult::Err(MSG_INVALID_FROM.to_string()),
    }
}

#[update(name = "setFee")]
#[candid_method(update, rename = "setFee")]
fn set_fee(fee: Fee) -> Result<bool, String> {
    TOKEN.write().unwrap().set_fee(&api::caller(), fee)
}

#[query(name = "setFeeTo")]
#[candid_method(update, rename = "setFeeTo")]
fn set_fee_to(fee_to: String) -> Result<bool, String> {
    match fee_to.parse::<TokenReceiver>() {
        Ok(holder) => TOKEN.write().unwrap().set_fee_to(&api::caller(), holder),
        Err(_) => api::trap(MSG_INVALID_FEE_TO),
    }
}

#[query(name = "tokenInfo")]
#[candid_method(query, rename = "tokenInfo")]
fn get_token_info() -> TokenInfo {
    let mut token_info = TOKEN.read().unwrap().token_info();
    token_info.cycles = api::canister_balance();
    token_info
}

#[query(name = "transactionByIndex")]
#[candid_method(query, rename = "transactionByIndex")]
fn transaction_by_index(tx_index: Nat) -> TxRecordResult {
    TOKEN.read().unwrap().transaction_by_index(&tx_index)
}

#[query(name = "lastTransactions")]
#[candid_method(query, rename = "lastTransactions")]
fn last_transactions(count: usize) -> Result<Vec<TxRecord>, String> {
    TOKEN.read().unwrap().last_transactions(count)
}

#[query(name = "transactionById")]
#[candid_method(query, rename = "transactionById")]
fn transaction_by_id(tx_id: String) -> TxRecordResult {
    TOKEN.read().unwrap().transaction_by_id(&tx_id)
}

#[update(name = "burnFrom")]
#[candid_method(update, rename = "burnFrom")]
async fn burn_from(
    from_sub_account: Option<Subaccount>,
    spender: String,
    value: Nat,
) -> TransactionResult {
    let caller = api::caller();
    let token_holder_owner = TokenHolder::new(caller, from_sub_account);
    let spender_parse_res = spender.parse::<TokenHolder>();
    match spender_parse_res {
        Ok(holder) => {
            // call token burn_from
            let tx_index = TOKEN.write().unwrap().burn_from(
                &caller,
                &token_holder_owner,
                &holder,
                value.clone(),
                api::time(),
            )?;

            let mut errors: Vec<String> = vec![];
            //exec auto-scaling storage strategy
            match exec_auto_scaling_strategy().await {
                Ok(_) => (),
                Err(e) => {
                    errors.push(e.to_string());
                }
            };

            TransactionResult::Ok(TransactionResponse {
                txid: encode_tx_id(api::id(), tx_index),
                error: if errors.len() > 0 { Some(errors) } else { None },
            })
        }

        Err(_) => TransactionResult::Err(MSG_INVALID_TO.to_string()),
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(from_sub_account: Option<Subaccount>, value: Nat) -> TransactionResult {
    let caller = api::caller();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let tx_index =
        TOKEN
            .write()
            .unwrap()
            .burn(&caller, &transfer_from, value.clone(), api::time())?;
    let mut errors: Vec<String> = vec![];
    //exec auto-scaling storage strategy
    match exec_auto_scaling_strategy().await {
        Ok(_) => (),
        Err(e) => {
            errors.push(e.to_string());
        }
    };

    TransactionResult::Ok(TransactionResponse {
        txid: encode_tx_id(api::id(), tx_index),
        error: if errors.len() > 0 { Some(errors) } else { None },
    })
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}

#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((TOKEN.read().unwrap().to_token_payload(),)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    // There can only be one value in stable memory, currently. otherwise, lifetime error.
    // https://docs.rs/ic-cdk/0.3.0/ic_cdk/storage/fn.stable_restore.html
    let (payload,): (TokenPayload,) = storage::stable_restore().unwrap();
    let mut token = TOKEN.write().unwrap();
    token.from_token_payload(payload);
}

// do something becore sending
fn before_token_sending(
    _transfer_from: &TokenHolder,
    _receiver: &TokenReceiver,
    _value: &Nat,
) -> Result<(), String> {
    Ok(())
}

// call it after transfer, notify receiver with (from,value)
async fn on_token_received(
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

async fn execute_call(receiver: &TokenReceiver, _call_data: CallData) -> Result<bool, String> {
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

async fn exec_auto_scaling_strategy() -> Result<(), String> {
    let token = TOKEN.read().unwrap();
    let inner_txs = token.get_inner_txs();
    let frist_tx_index_inner = token.get_tx_index(&inner_txs[0]);
    // When create auto-scaling storage ?
    // DFT's txs count > 2000
    // It's means when creating a test DFT, when the number of transactions is less than 2000, no storage will be created to save cycles
    if inner_txs.len() >= MAX_TXS_CACHE_IN_DFT * 2 {
        let storage_canister_id_res =
            get_or_create_available_storage_id(&frist_tx_index_inner).await;

        match storage_canister_id_res {
            Ok(storage_canister_id) => {
                let should_save_txs = inner_txs[0..MAX_TXS_CACHE_IN_DFT].to_vec();
                //save the txs to auto-scaling storage
                match api::call::call(storage_canister_id, "batchAppend", (should_save_txs,)).await
                {
                    Ok((res,)) => {
                        if res {
                            let mut token = TOKEN.write().unwrap();
                            (0..MAX_TXS_CACHE_IN_DFT).for_each(|_| token.remove_inner_txs(0));
                        }
                    }
                    Err((_, emsg)) => {
                        api::print(format!(
                            "batchAppend: save to auto-scaling storage failed,{}  ",
                            emsg
                        ));
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
                return Err(emsg);
            }
        };
    }

    Ok(())
}

async fn get_or_create_available_storage_id(tx_index: &Nat) -> Result<Principal, String> {
    let mut max_key = Nat::from(0);
    let mut last_storage_id = Principal::anonymous();
    for (k, v) in &TOKEN.read().unwrap().get_storage_canister_ids() {
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
                return Err(MSG_STORAGE_SCALING_FAILED.to_string());
            }
        };
    }

    if is_necessary_create_new_storage_canister {
        const STORAGE_WASM: &[u8] = std::include_bytes!(
            "../../target/wasm32-unknown-unknown/release/dft_tx_storage_opt.wasm"
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
                        TOKEN
                            .write()
                            .unwrap()
                            .add_storage_canister_ids(tx_index.clone(), cdr.canister_id);
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
