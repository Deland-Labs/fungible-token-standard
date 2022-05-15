use crate::service;
use crate::types::StorageInfo;
use candid::Principal;
use candid::{candid_method, Nat};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;

#[init]
#[candid_method(init)]
fn canister_init(dft_id: Principal, dft_tx_start_index: Nat) {
    service::init(dft_id, dft_tx_start_index.0, api::time());
}

#[update(name = "batchAppend")]
#[candid_method(update, rename = "batchAppend")]
fn batch_append(blocks: Vec<EncodedBlock>) -> BooleanResult {
    match service::batch_append(&api::caller(), blocks, api::time()) {
        Ok(_) => BooleanResult::Ok(true),
        Err(e) => BooleanResult::Err(e.into()),
    }
}

#[query(name = "blockByHeight")]
#[candid_method(query, rename = "blockByHeight")]
fn block_by_index(block_height: Nat) -> BlockResult {
    service::get_block_by_height(block_height.0)
}

#[query(name = "blocksByQuery")]
#[candid_method(query, rename = "blocksByQuery")]
fn blocks(block_height_start: Nat, size: usize) -> BlockListResult {
    service::get_blocks_by_query(block_height_start.0, size)
}

#[query(name = "storageInfo")]
#[candid_method(query, rename = "storageInfo")]
fn storage_info() -> StorageInfo {
    let mut info = service::get_storage_info();
    info.cycles = api::canister_balance();
    info
}
