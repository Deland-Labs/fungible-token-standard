use crate::token_fee::CandidTokenFee;

use super::TokenFee;
use candid::{CandidType, Deserialize};
use getset::{Getters, Setters};
use serde::Serialize;

#[derive(Getters, Setters)]
#[getset(get = "pub")]
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
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

#[derive(Getters, Setters)]
#[getset(get = "pub")]
#[derive(CandidType, Clone, Default, Debug, Deserialize)]
pub struct CandidTokenMetadata {
    name: String,
    symbol: String,
    decimals: u8,
    #[getset(set = "pub")]
    fee: CandidTokenFee,
}

impl From<TokenMetadata> for CandidTokenMetadata {
    fn from(token_metadata: TokenMetadata) -> Self {
        Self {
            name: token_metadata.name,
            symbol: token_metadata.symbol,
            decimals: token_metadata.decimals,
            fee: token_metadata.fee.into(),
        }
    }
}
