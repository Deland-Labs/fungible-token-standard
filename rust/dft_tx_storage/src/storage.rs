use std::convert::TryInto;

use candid::{candid_method, Nat};
use candid::{CandidType, Deserialize, Principal};
use dft_types::*;
use dft_utils::decode_tx_id;
use ic_cdk::{api, storage};
use ic_cdk_macros::*;
use std::sync::RwLock;

static mut DFT_ID: Principal = Principal::anonymous();

const MSG_OUT_OF_TX_INDEX_RANGE: &str = "DFT_TX_STORAGE: out of tx index range";
const MSG_INVALID_TX_ID: &str = "DFT_TX_STORAGE: invalid tx id";
const MSG_NOT_BELONG_DFT_TX_ID: &str = "DFT_TX_STORAGE: tx id not belong to the current dft";

lazy_static! {
    static ref DFT_TX_START_INDEX: RwLock<Nat> = RwLock::new(Nat::from(0));
}

#[derive(CandidType, Debug, Deserialize)]
pub struct StorageInfo {
    pub dft_id: Principal,
    pub tx_start_index: Nat,
    pub txs_count: Nat,
    pub cycles: u64,
}

#[derive(CandidType, Debug, Deserialize)]
pub struct StoragePayload {
    pub dft_id: Principal,
    pub tx_start_index: Nat,
    pub txs: Txs,
}

#[init]
fn canister_init(dft_id: Principal, dft_tx_start_index: Nat) {
    unsafe {
        DFT_ID = dft_id;
        *DFT_TX_START_INDEX.write().unwrap() = dft_tx_start_index.clone();
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

#[query(name = "transactionByIndex")]
#[candid_method(query, rename = "transactionByIndex")]
fn transaction_by_index(tx_index: Nat) -> Result<TxRecord, String> {
    let txs = storage::get::<Txs>();
    let rw_start_index = DFT_TX_START_INDEX.read().unwrap().clone();

    if tx_index < rw_start_index || tx_index > (rw_start_index.clone() + txs.len() as u128) {
        Err(MSG_OUT_OF_TX_INDEX_RANGE.to_string())
    } else {
        let inner_index: usize = (tx_index - rw_start_index).0.try_into().unwrap();
        Ok(txs[inner_index].clone())
    }
}

#[query(name = "transactions")]
#[candid_method(query, rename = "transactions")]
fn transactions(tx_start_index: Nat, size: usize) -> Result<Vec<TxRecord>, String> {
    let txs = storage::get::<Txs>();
    let mut ret: Vec<TxRecord> = Vec::new();
    let rw_start_index = DFT_TX_START_INDEX.read().unwrap().clone();

    if tx_start_index < rw_start_index
        || tx_start_index > (rw_start_index.clone() + txs.len() as u128)
    {
        Err(MSG_OUT_OF_TX_INDEX_RANGE.to_string())
    } else {
        let max_index = txs.len();
        let mut start_index: usize = (tx_start_index - rw_start_index.clone())
            .0
            .try_into()
            .unwrap();
        let end_index = start_index + size;
        while start_index < end_index && start_index < max_index {
            ret.push(txs[start_index].clone());
            start_index = start_index + 1;
        }
        Ok(ret)
    }
}

#[query(name = "transactionById")]
#[candid_method(query, rename = "transactionById")]
fn get_transaction_by_id(tx_id: String) -> Result<TxRecord, String> {
    let decode_res = decode_tx_id(tx_id);
    match decode_res {
        Ok((dft_id, tx_index)) => unsafe {
            if dft_id != DFT_ID {
                Err(MSG_NOT_BELONG_DFT_TX_ID.to_string())
            } else {
                transaction_by_index(tx_index)
            }
        },
        Err(_) => Err(MSG_INVALID_TX_ID.to_string()),
    }
}

#[query(name = "storageInfo")]
#[candid_method(query, rename = "storageInfo")]
fn storage_info() -> StorageInfo {
    StorageInfo {
        dft_id: unsafe { DFT_ID },
        tx_start_index: DFT_TX_START_INDEX.read().unwrap().clone(),
        txs_count: storage::get::<Txs>().len().into(),
        cycles: api::canister_balance().into(),
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
    let rw_start_index = DFT_TX_START_INDEX.read().unwrap().clone();
    let dft_id = unsafe { DFT_ID };
    let tx_start_index = rw_start_index;
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
        *DFT_TX_START_INDEX.write().unwrap() = payload.tx_start_index;
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

#[test]
fn check_nat_tyr_into_usize() {
    let origin_val: usize = 4294967295;
    let nat = Nat::from(4294967295u32);
    let usize: usize = nat.0.try_into().unwrap();

    assert_eq!(origin_val, usize, "can not convert nat to usize");
}
