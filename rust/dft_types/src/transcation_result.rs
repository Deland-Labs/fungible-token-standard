use super::TransactionId;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TransactionResponse {
    pub txid: TransactionId,
    pub error: Option<Vec<String>>,
}

pub type TransactionResult = Result<TransactionResponse, String>;
