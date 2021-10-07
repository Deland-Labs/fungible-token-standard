use super::TransactionId;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TransactionResponse {
    pub txid: TransactionId,
    pub error: Option<Vec<String>>,
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TransactionResult {
    //transfer succeed, but call failed & notify failed
    Ok(TransactionResponse),
    Err(String),
}
