use crate::{types::*, utils::decode_tx_id};
use candid::candid_method;
use ic_cdk::{api, export::Principal, storage};
use ic_cdk_macros::*;

static mut DFT_ID: Principal = Principal::anonymous();
static mut DFT_TX_START_INDEX: u128 = 0;

const MSG_OUT_OF_TX_INDEX_RANGE: &str = "DFT_TX_STORAGE: out of tx index range";
const MSG_INVALID_TX_ID: &str = "DFT_TX_STORAGE: invalid tx id";
const MSG_NOT_BELONG_DFT_TX_ID: &str = "DFT_TX_STORAGE: tx id not belong to the current dft";

#[init]
fn canister_init(dft_id: Principal, dft_tx_start_index: u128) {
    unsafe {
        DFT_ID = dft_id;
        DFT_TX_START_INDEX = dft_tx_start_index;
    }
    api::print(format!(
        "dft is {} start index is {}",
        dft_id, dft_tx_start_index
    ));
}

#[update(name = "append")]
#[candid_method(update, rename = "append")]
fn append(tx: TxRecord) -> bool {
    _only_allow_token_canister();
    let txs_storage = storage::get_mut::<Txs>();
    txs_storage.push(tx);
    true
}

#[update(name = "batchAppend")]
#[candid_method(update, rename = "batchAppend")]
fn batch_append(txs: Vec<TxRecord>) -> bool {
    _only_allow_token_canister();
    let txs_storage = storage::get_mut::<Txs>();
    txs_storage.extend(txs);
    true
}

#[query(name = "getTransactionByIndex")]
#[candid_method(query, rename = "getTransactionByIndex")]
fn get_transaction_by_index(tx_index: u128) -> Result<TxRecord, String> {
    let txs = storage::get::<Txs>();
    unsafe {
        if tx_index < DFT_TX_START_INDEX || tx_index > (DFT_TX_START_INDEX + txs.len() as u128) {
            Err(MSG_OUT_OF_TX_INDEX_RANGE.to_string())
        } else {
            Ok(txs[(tx_index - DFT_TX_START_INDEX) as usize].clone())
        }
    }
}

#[query(name = "getTransactions")]
#[candid_method(query, rename = "getTransactions")]
fn get_transactions(tx_start_index: u128, size: usize) -> Result<Vec<TxRecord>, String> {
    let txs = storage::get::<Txs>();
    let mut ret: Vec<TxRecord> = Vec::new();
    unsafe {
        if tx_start_index < DFT_TX_START_INDEX
            || tx_start_index > (DFT_TX_START_INDEX + txs.len() as u128)
        {
            Err(MSG_OUT_OF_TX_INDEX_RANGE.to_string())
        } else {
            let max_index = txs.len();
            let mut start_index = (tx_start_index - DFT_TX_START_INDEX) as usize;
            let end_index = start_index + size;
            while start_index < end_index && start_index < max_index {
                ret.push(txs[start_index].clone());
                start_index = start_index + 1;
            }
            Ok(ret)
        }
    }
}

#[query(name = "getTransactionById")]
#[candid_method(query, rename = "getTransactionById")]
fn get_transaction_by_id(tx_id: String) -> Result<TxRecord, String> {
    let decode_res = decode_tx_id(tx_id);
    match decode_res {
        Ok((dft_id, tx_index)) => unsafe {
            if dft_id != DFT_ID {
                Err(MSG_NOT_BELONG_DFT_TX_ID.to_string())
            } else {
                get_transaction_by_index(tx_index)
            }
        },
        Err(_) => Err(MSG_INVALID_TX_ID.to_string()),
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
    let dft_id = unsafe { DFT_ID };
    let tx_start_index = unsafe { DFT_TX_START_INDEX };
    let txs = storage::get::<Txs>();

    let payload = StoragePayload {
        dft_id,
        tx_start_index,
        txs: txs.to_vec(),
    };
    storage::stable_save((payload,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    // There can only be one value in stable memory, currently. otherwise, lifetime error.
    // https://docs.rs/ic-cdk/0.3.0/ic_cdk/storage/fn.stable_restore.html
    let (payload,): (StoragePayload,) = storage::stable_restore().unwrap();
    unsafe {
        DFT_ID = payload.dft_id;
        DFT_TX_START_INDEX = payload.tx_start_index;
    }

    let txs = storage::get_mut::<Txs>();
    for k in payload.txs {
        txs.push(k);
    }
}

fn _only_allow_token_canister() {
    unsafe {
        if DFT_ID != api::caller() {
            api::trap(format!("DFT_TX_STORAGE: only allow dft {}", DFT_ID.to_text()).as_str());
        }
    }
}
