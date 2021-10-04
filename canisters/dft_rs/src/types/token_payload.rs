use super::{MetaData, TokenHolder};
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use std::string::String;

#[derive(CandidType, Debug, Deserialize)]
pub struct TokenPayload {
    pub owner: Principal,
    pub fee_to: TokenHolder,
    pub meta: MetaData,
    pub extend: Vec<(String, String)>,
    pub logo: Vec<u8>,
    pub balances: Vec<(TokenHolder, u128)>,
    pub allowances: Vec<(TokenHolder, Vec<(TokenHolder, u128)>)>,
    pub tx_id_cursor: u128,
    pub storage_canister_id: Principal,
}
