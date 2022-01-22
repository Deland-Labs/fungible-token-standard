use candid::{CandidType, Deserialize, Nat};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenMetrics {
    pub holders: Nat,
    #[serde(rename = "allowanceSize")]
    pub allowance_size: Nat,
    #[serde(rename = "totalTxCount")]
    pub total_tx_count: Nat,
    #[serde(rename = "innerTxCount")]
    pub inner_tx_count: Nat
}