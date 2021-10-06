use super::TransactionId;
use ic_cdk::export::candid::{CandidType, Deserialize};


#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct BurnResponse {
    pub txid: TransactionId,
    pub error: Option<Vec<String>>,
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum BurnResult {
    Ok(BurnResponse),
    Err(String),
}
