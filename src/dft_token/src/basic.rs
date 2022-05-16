use candid::{candid_method, Nat};
use dft_basic::auto_scaling_storage::AutoScalingStorageService;
use dft_basic::service::basic_service;
use dft_types::*;
use dft_utils::ic_logger::ICLogger;
use ic_cdk::api::{data_certificate, set_certified_data};
use ic_cdk::{api, export::Principal};
use ic_cdk_macros::*;
use std::string::String;
use std::sync::Once;

static INIT: Once = Once::new();

pub(crate) fn canister_module_init() {
    INIT.call_once(|| {
        ICLogger::init();
    });
}

#[init]
#[candid_method(init)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::clone_on_copy)]
async fn canister_init(
    sub_account: Option<Subaccount>,
    logo: Option<Vec<u8>>,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: Nat,
    fee: CandidTokenFee,
    caller: Option<Principal>,
    archive_option: Option<ArchiveOptions>,
) {
    canister_module_init();
    let real_caller = caller.unwrap_or_else(api::caller);
    let owner_holder = TokenHolder::new(real_caller, sub_account);

    basic_service::token_initialize(
        &real_caller,
        api::id(),
        logo,
        name,
        symbol,
        decimals,
        fee.into(),
        owner_holder.clone(),
        archive_option,
    );
    if total_supply == 0u32 {
        return;
    }
    if let Ok((_, block_hash, _)) = dft_mintable::mint(
        &real_caller,
        &owner_holder,
        total_supply.0,
        None,
        api::time(),
    ) {
        set_certified_data(&block_hash);
    }
}

#[query(name = "owner")]
#[candid_method(query, rename = "owner")]
fn owner() -> Principal {
    basic_service::owner()
}

#[query(name = "name")]
#[candid_method(query, rename = "name")]
fn get_name() -> String {
    basic_service::name()
}

#[query(name = "symbol")]
#[candid_method(query, rename = "symbol")]
fn get_symbol() -> String {
    basic_service::symbol()
}

#[query(name = "decimals")]
#[candid_method(query, rename = "decimals")]
fn get_decimals() -> u8 {
    basic_service::decimals()
}

#[query(name = "totalSupply")]
#[candid_method(query, rename = "totalSupply")]
fn get_total_supply() -> Nat {
    basic_service::total_supply().into()
}

#[query(name = "fee")]
#[candid_method(query, rename = "fee")]
fn get_fee_setting() -> CandidTokenFee {
    basic_service::fee().into()
}

#[query(name = "meta")]
#[candid_method(query, rename = "meta")]
fn get_meta_data() -> CandidTokenMetadata {
    basic_service::metadata().into()
}

#[query(name = "desc")]
#[candid_method(query, rename = "desc")]
fn get_desc_info() -> Vec<(String, String)> {
    basic_service::desc()
        .iter()
        .map(|v| (v.0.clone(), v.1.clone()))
        .collect()
}

#[query(name = "logo")]
#[candid_method(query, rename = "logo")]
fn logo() -> Vec<u8> {
    basic_service::logo().unwrap_or_default()
}

#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(holder: String) -> Nat {
    let token_holder_parse_result = holder.parse::<TokenHolder>();
    match token_holder_parse_result {
        Ok(token_holder) => basic_service::balance_of(&token_holder).into(),
        _ => 0u32.into(),
    }
}

#[query(name = "allowance")]
#[candid_method(query, rename = "allowance")]
fn allowance(owner: String, spender: String) -> Nat {
    let token_holder_owner_parse_result = owner.parse::<TokenHolder>();
    let token_holder_spender_parse_result = spender.parse::<TokenHolder>();

    if let Ok(token_holder_owner) = token_holder_owner_parse_result {
        if let Ok(token_holder_spender) = token_holder_spender_parse_result {
            return basic_service::allowance(&token_holder_owner, &token_holder_spender).into();
        }
    }

    0u32.into()
}

#[update(name = "approve")]
#[candid_method(update, rename = "approve")]
async fn approve(
    owner_sub_account: Option<Subaccount>,
    spender: String,
    value: Nat,
    created_at: Option<u64>,
) -> OperationResult {
    let caller = api::caller();
    let token_id = api::id();
    let owner_holder = TokenHolder::new(caller, owner_sub_account);
    match spender.parse::<TokenHolder>() {
        Ok(spender_holder) => {
            match basic_service::approve(
                &caller,
                &owner_holder,
                &spender_holder,
                value.0,
                created_at,
                api::time(),
            ) {
                Ok((block_height, block_hash, tx_hash)) => {
                    set_certified_data(&block_hash);
                    let tx_id = hex::encode(tx_hash.as_ref());
<<<<<<< HEAD
                    let auto_scaling_service = AutoScalingStorageService::new(token_id);
=======
                    let auto_scaling_service = AutoScalingStorageService::new();
>>>>>>> ebc4cf1 (Refactor: auto_scaling_storage for unit test)
                    auto_scaling_service.exec_auto_scaling_strategy().await;
                    OperationResult::Ok {
                        tx_id,
                        block_height: block_height.into(),
                    }
                }
                Err(e) => OperationResult::Err(e.into()),
            }
        }
        Err(_) => OperationResult::Err(DFTError::InvalidSpender.into()),
    }
}

#[query(name = "allowancesOf")]
#[candid_method(query, rename = "allowancesOf")]
fn allowances_of_holder(holder: String) -> Vec<(TokenHolder, Nat)> {
    match holder.parse::<TokenHolder>() {
        Ok(token_holder) => basic_service::allowances_of(&token_holder)
            .into_iter()
            .map(|(v, n)| (v, n.into()))
            .collect(),
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
    created_at: Option<u64>,
) -> OperationResult {
    let caller = api::caller();
    let token_id = api::id();
    let now = api::time();
    let spender = TokenHolder::new(caller, spender_sub_account);

    match from.parse::<TokenHolder>() {
        Ok(from_token_holder) => match to.parse::<TokenHolder>() {
            Ok(to_token_holder) => {
                // exec before-transfer check :before_token_sending
                if let Err(e) = before_token_sending(&from_token_holder, &to_token_holder, &value.0)
                {
                    return OperationResult::Err(e);
                }
                match basic_service::transfer_from(
                    &caller,
                    &from_token_holder,
                    &spender,
                    &to_token_holder,
                    value.0.clone(),
                    created_at,
                    now,
                ) {
                    Ok((block_height, block_hash, tx_hash)) => {
                        set_certified_data(&block_hash);
<<<<<<< HEAD
                        let auto_scaling_service = AutoScalingStorageService::new(token_id);
=======
                        let auto_scaling_service = AutoScalingStorageService::new();
>>>>>>> ebc4cf1 (Refactor: auto_scaling_storage for unit test)
                        auto_scaling_service.exec_auto_scaling_strategy().await;
                        OperationResult::Ok {
                            tx_id: hex::encode(tx_hash.as_ref()),
                            block_height: block_height.into(),
                        }
                    }
                    Err(e) => OperationResult::Err(e.into()),
                }
            }
            _ => OperationResult::Err(DFTError::InvalidArgFormatTo.into()),
        },
        _ => OperationResult::Err(DFTError::InvalidArgFormatFrom.into()),
    }
}

#[update(name = "transfer")]
#[candid_method(update, rename = "transfer")]
async fn transfer(
    from_sub_account: Option<Subaccount>,
    to: String,
    value: Nat,
    created_at: Option<u64>,
) -> OperationResult {
    let caller = api::caller();
    let token_id = api::id();
    let now = api::time();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let receiver_parse_result = to.parse::<TokenReceiver>();

    match receiver_parse_result {
        Ok(receiver) => {
            //exec before-transfer check
            if let Err(e) = before_token_sending(&transfer_from, &receiver, &value.0) {
                return OperationResult::Err(e);
            };
            //transfer token
            match basic_service::transfer(
                &caller,
                &transfer_from,
                &receiver,
                value.0.clone(),
                created_at,
                now,
            ) {
                Ok((block_height, block_hash, tx_hash)) => {
                    set_certified_data(&block_hash);
<<<<<<< HEAD
                    let auto_scaling_service = AutoScalingStorageService::new(token_id);
=======
                    let auto_scaling_service = AutoScalingStorageService::new();
>>>>>>> ebc4cf1 (Refactor: auto_scaling_storage for unit test)
                    auto_scaling_service.exec_auto_scaling_strategy().await;
                    OperationResult::Ok {
                        tx_id: hex::encode(tx_hash.as_ref()),
                        block_height: block_height.into(),
                    }
                }
                Err(e) => OperationResult::Err(e.into()),
            }
        }
        _ => OperationResult::Err(DFTError::InvalidArgFormatTo.into()),
    }
}

#[query(name = "tokenInfo")]
#[candid_method(query, rename = "tokenInfo")]
fn get_token_info() -> TokenInfo {
    let mut token_info = basic_service::token_info();
    token_info.certificate = data_certificate().map(serde_bytes::ByteBuf::from);
    token_info.cycles = api::canister_balance();
    token_info
}

#[query(name = "blockByHeight")]
#[candid_method(query, rename = "blockByHeight")]
fn block_by_height(block_height: Nat) -> BlockResult {
    basic_service::block_by_height(block_height.0)
}

#[query(name = "blocksByQuery")]
#[candid_method(query, rename = "blocksByQuery")]
fn blocks_by_query(start: Nat, count: usize) -> QueryBlocksResult {
    let mut res = basic_service::blocks_by_query(start.0, count);
    res.certificate = data_certificate().map(serde_bytes::ByteBuf::from);
    res
}

#[query(name = "archives")]
#[candid_method(query, rename = "archives")]
fn archives() -> Vec<ArchiveInfo> {
    basic_service::archives()
}

// do something before sending
fn before_token_sending(
    _transfer_from: &TokenHolder,
    _receiver: &TokenReceiver,
    _value: &TokenAmount,
) -> ActorResult<()> {
    Ok(())
}
