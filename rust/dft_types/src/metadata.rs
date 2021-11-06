use super::Fee;
use candid::{CandidType, Deserialize, Nat};

#[derive(CandidType, Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    #[serde(rename = "totalSupply")]
    pub total_supply: Nat,
    pub fee: Fee,
}
