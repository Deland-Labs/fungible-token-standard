use super::TokenFee;
use candid::{CandidType, Deserialize};
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
#[getset(get = "pub")]
#[derive(CandidType, Clone, Default, Debug, Deserialize)]
pub struct TokenMetadata {
    name: String,
    symbol: String,
    decimals: u8,
    #[getset(set = "pub")]
    fee: TokenFee,
}

impl TokenMetadata {
    // new
    pub fn new(name: String, symbol: String, decimals: u8, fee: TokenFee) -> Self {
        Self {
            name,
            symbol,
            decimals,
            fee,
        }
    }
}
