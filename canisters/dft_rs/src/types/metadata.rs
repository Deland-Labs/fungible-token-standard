use super::Fee;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub fee: Fee,
}
