use super::TransactionId;
use candid::{CandidType, Deserialize};
use crate::{ActorError};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TransactionResponse {
    #[serde(rename="txId")]
    pub tx_id: TransactionId,
    pub error: Option<ActorError>,
}