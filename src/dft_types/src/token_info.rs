use super::{TokenHolder, CandidTokenFee};
use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenInfo {
    pub owner: Principal,
    #[serde(rename = "chainLength")]
    pub chain_length: Nat,
    pub holders: usize,
    #[serde(rename = "allowanceSize")]
    pub allowance_size: usize,
    #[serde(rename = "feeTo")]
    pub fee_to: TokenHolder,
    pub fee: CandidTokenFee,
    #[serde(rename = "archiveCanisters")]
    pub archive_canisters: Vec<Principal>,
    pub certificate: Option<serde_bytes::ByteBuf>,
}
