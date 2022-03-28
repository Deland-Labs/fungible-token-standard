use super::TokenHolder;
use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenInfo {
    pub owner: Principal,
    pub holders: Nat,
    #[serde(rename="allowanceSize")]
    pub allowance_size: Nat,
    #[serde(rename="feeTo")]
    pub fee_to: TokenHolder,
    #[serde(rename="txCount")]
    pub tx_count: Nat,
    pub cycles: u64,
    pub storages: Vec<Principal>,
}
