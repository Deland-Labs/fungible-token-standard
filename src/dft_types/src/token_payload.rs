use super::{TokenHolder, TokenMetadata, Txs};
use candid::{CandidType, Deserialize, Nat, Principal};
use std::string::String;

#[derive(CandidType, Debug, Deserialize)]
pub struct TokenPayload {
    pub token_id: Principal,
    pub owner: Principal,
    pub fee_to: TokenHolder,
    pub meta: TokenMetadata,
    pub desc: Vec<(String, String)>,
    pub logo: Vec<u8>,
    pub balances: Vec<(TokenHolder, Nat)>,
    pub allowances: Vec<(TokenHolder, Vec<(TokenHolder, Nat)>)>,
    pub tx_index_cursor: Nat,
    pub storage_canister_ids: Vec<(Nat, Principal)>,
    pub txs_inner: Txs,
}
