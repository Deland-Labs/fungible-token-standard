use super::{MetaData, TokenHolder};
use ic_cdk::export::{
    candid::{CandidType, Deserialize, Nat},
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
    pub balances: Vec<(TokenHolder, Nat)>,
    pub allowances: Vec<(TokenHolder, Vec<(TokenHolder, Nat)>)>,
    pub tx_id_cursor: Nat,
    pub storage_canister_id: Principal,
}
