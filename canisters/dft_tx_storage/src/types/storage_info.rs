use candid::Nat;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};

#[derive(CandidType, Debug, Deserialize)]
pub struct StorageInfo {
    pub dft_id: Principal,
    pub tx_start_index: Nat,
    pub txs_count: Nat,
    pub cycles: u64,
}
