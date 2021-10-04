use super::TransactionId;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct ApproveResponse {
    pub txid: TransactionId,
    pub error: Option<String>,
}

// Invalid data: Invalid IDL blob by candid 0.6.21
#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum ApproveResult {
    Ok(ApproveResponse),
    Err(String),
}
