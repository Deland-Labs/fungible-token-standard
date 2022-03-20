use crate::state::STORAGE;
use crate::storage::StorageInfo;
use candid::{Principal};
use candid::{candid_method, Nat};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;

#[init]
fn canister_init(dft_id: Principal, dft_tx_start_index: Nat) {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.initialize(dft_id.clone(), dft_tx_start_index.clone());
    });
}

#[update(name = "append")]
#[candid_method(update, rename = "append")]
fn append(tx: TxRecord) -> BooleanResult {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.append(&api::caller(), tx).into()
    })
}

#[update(name = "batchAppend")]
#[candid_method(update, rename = "batchAppend")]
fn batch_append(txs: Vec<TxRecord>) -> BooleanResult {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.batch_append(&api::caller(), txs).into()
    })
}

#[query(name = "transactionByIndex")]
#[candid_method(query, rename = "transactionByIndex")]
fn transaction_by_index(tx_index: Nat) -> TxRecordResult {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get_tx_by_index(tx_index).into()
    })
}

#[query(name = "transactions")]
#[candid_method(query, rename = "transactions")]
fn transactions(tx_start_index: Nat, size: usize) -> TxRecordListResult {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get_tx_by_index_range(tx_start_index, size).into()
    })
}

#[query(name = "transactionById")]
#[candid_method(query, rename = "transactionById")]
fn get_transaction_by_id(tx_id: String) -> TxRecordResult {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get_tx_by_id(tx_id).into()
    })
}

#[query(name = "storageInfo")]
#[candid_method(query, rename = "storageInfo")]
fn storage_info() -> StorageInfo {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get_storage_info()
    })
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}
