use candid::{CandidType, Deserialize, Nat};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenMetrics {
    pub holders: Nat,
    #[serde(rename = "allowanceSize")]
    pub allowance_size: Nat,
    #[serde(rename = "totalBlockCount")]
    pub total_block_count: Nat,
    #[serde(rename = "localBlockCount")]
    pub local_block_count: Nat,
}
