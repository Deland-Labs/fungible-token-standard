use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};

use super::TokenHolder;

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenInfo {
    pub owner: Principal,
    pub holders: u128,
    pub fee_to: TokenHolder,
    pub tx_count: u128,
    pub cycles: u64,
}
