use super::TransactionId;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum BurnResult {
    Ok(TransactionId),
    Err(String),
}
