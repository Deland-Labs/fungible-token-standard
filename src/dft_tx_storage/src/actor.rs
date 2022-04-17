use crate::state::STORAGE;
use crate::storage::StorageInfo;
use candid::Principal;
use candid::{candid_method, Nat};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;

#[init]
#[candid_method(init)]
fn canister_init(dft_id: Principal, dft_tx_start_index: Nat) {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.initialize(dft_id, dft_tx_start_index);
    });
}

#[update(name = "batchAppend")]
#[candid_method(update, rename = "batchAppend")]
fn batch_append(blocks: Vec<EncodedBlock>) -> BooleanResult {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.batch_append(&api::caller(), blocks).into()
    })
}

#[query(name = "blockByHeight")]
#[candid_method(query, rename = "blockByHeight")]
fn block_by_index(block_height: Nat) -> BlockResult {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get_block_by_height(block_height)
    })
}

#[query(name = "blocksByQuery")]
#[candid_method(query, rename = "blocksByQuery")]
fn blocks(block_height_start: Nat, size: usize) -> BlockListResult {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get_blocks_by_query(block_height_start, size)
    })
}

#[query(name = "storageInfo")]
#[candid_method(query, rename = "storageInfo")]
fn storage_info() -> StorageInfo {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        let mut storage_info = storage.get_storage_info();
        storage_info.cycles = api::canister_balance();
        storage_info
    })
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}
