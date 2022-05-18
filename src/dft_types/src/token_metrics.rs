use candid::{CandidType, Deserialize, Nat};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenMetrics {
    pub holders: usize,
    #[serde(rename = "allowanceSize")]
    pub allowance_size: usize,
    #[serde(rename = "chainLength")]
    pub chain_length: Nat,
    #[serde(rename = "localBlockCount")]
    pub local_block_count: Nat,
    #[serde(rename = "cyclesBalance")]
    pub cycles_balance: Nat,
    pub certificate: Option<serde_bytes::ByteBuf>,
}
