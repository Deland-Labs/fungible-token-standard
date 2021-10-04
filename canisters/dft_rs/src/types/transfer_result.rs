use super::TransactionId;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TransferResponse {
    pub txid: TransactionId,
    pub error: Option<Vec<String>>,
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TransferResult {
    //transfer succeed, but call failed & notify failed
    Ok(TransferResponse),
    Err(String),
}
