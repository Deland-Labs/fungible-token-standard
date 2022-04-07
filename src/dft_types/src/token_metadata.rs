use super::Fee;
use candid::{CandidType, Deserialize, Nat};
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
#[getset(get = "pub")]
#[derive(CandidType, Clone, Default, Debug, Deserialize)]
pub struct TokenMetadata {
    name: String,
    symbol: String,
    decimals: u8,
    #[getset(set = "pub")]
    fee: Fee,
}

impl TokenMetadata {
    // new
    pub fn new(name: String, symbol: String, decimals: u8, fee: Fee) -> Self {
        Self {
            name,
            symbol,
            decimals,
            fee,
        }
    }
}
