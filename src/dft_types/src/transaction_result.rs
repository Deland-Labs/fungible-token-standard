use super::TransactionId;
use crate::ErrorInfo;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Default, Clone, Deserialize)]
pub struct TransactionResponse {
    #[serde(rename = "txId")]
    pub tx_id: TransactionId,
    pub error: Option<ErrorInfo>,
}
