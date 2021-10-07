use super::Fee;
use candid::{CandidType, Deserialize, Nat};

#[derive(CandidType, Debug, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Nat,
    pub fee: Fee,
}
