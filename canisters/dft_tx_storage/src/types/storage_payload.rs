use super::Txs;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};

#[derive(CandidType, Debug, Deserialize)]
pub struct StoragePayload {
    pub dft_id: Principal,
    pub tx_start_index: u128,
    pub txs: Txs,
}
