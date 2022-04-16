use super::TokenHolder;
use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenInfo {
    pub owner: Principal,
    pub holders: usize,
    #[serde(rename = "allowanceSize")]
    pub allowance_size: usize,
    #[serde(rename = "feeTo")]
    pub fee_to: TokenHolder,
    #[serde(rename = "blockHeight")]
    pub block_height: Nat,
    pub storages: Vec<Principal>,
    pub cycles: u64,
    pub certificate: Option<serde_bytes::ByteBuf>,
}
