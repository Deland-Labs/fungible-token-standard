use crate::TokenHolder;

use super::Fee;
use candid::{CandidType, Deserialize, Nat};
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
#[getset(get = "pub")]
#[derive(CandidType, Clone, Default, Debug, Deserialize)]
pub struct Metadata {
    name: String,
    symbol: String,
    decimals: u8,
    #[getset(set = "pub")]
    #[serde(rename = "totalSupply")]
    total_supply: Nat,
    #[getset(set = "pub")]
    fee: Fee,
}

impl Metadata {
    // new
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: Nat, fee: Fee) -> Self {
        Self {
            name,
            symbol,
            decimals,
            total_supply,
            fee,
        }
    }
}
