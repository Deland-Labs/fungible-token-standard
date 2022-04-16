use candid::{CandidType, Deserialize, Nat};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenMetrics {
    pub holders: usize,
    #[serde(rename = "allowanceSize")]
    pub allowance_size: usize,
    #[serde(rename = "totalBlockCount")]
    pub total_block_count: Nat,
    #[serde(rename = "localBlockCount")]
    pub local_block_count: Nat,
}
