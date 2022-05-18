use crate::token_fee::TokenFee;

use super::InnerTokenFee;
use candid::{CandidType, Deserialize};
use getset::{Getters, Setters};
use serde::Serialize;

#[derive(Getters, Setters)]
#[getset(get = "pub")]
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct InnerTokenMetadata {
    name: String,
    symbol: String,
    decimals: u8,
    #[getset(set = "pub")]
    fee: InnerTokenFee,
}

impl InnerTokenMetadata {
    // new
    pub fn new(name: String, symbol: String, decimals: u8, fee: InnerTokenFee) -> Self {
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
pub struct TokenMetadata {
    name: String,
    symbol: String,
    decimals: u8,
    #[getset(set = "pub")]
    fee: TokenFee,
}

impl From<InnerTokenMetadata> for TokenMetadata {
    fn from(token_metadata: InnerTokenMetadata) -> Self {
        Self {
            name: token_metadata.name,
            symbol: token_metadata.symbol,
            decimals: token_metadata.decimals,
            fee: token_metadata.fee.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_token_metadata_new() {
        let token_metadata = InnerTokenMetadata::new(
            "name".to_string(),
            "symbol".to_string(),
            1,
            InnerTokenFee::new(1u32.into(), 2, 3),
        );
        assert_eq!(token_metadata.name, "name");
        assert_eq!(token_metadata.symbol, "symbol");
        assert_eq!(token_metadata.decimals, 1u8);
        assert_eq!(token_metadata.fee.minimum, BigUint::from(1u32));
        assert_eq!(token_metadata.fee.rate, 2);
        assert_eq!(token_metadata.fee.rate_decimals, 3);
    }

    #[test]
    fn test_to_candid_type() {
        let token_metadata = InnerTokenMetadata::new(
            "name".to_string(),
            "symbol".to_string(),
            1,
            InnerTokenFee::new(1u32.into(), 2, 3),
        );

        let candid_token_metadata: TokenMetadata = token_metadata.clone().into();

        assert_eq!(candid_token_metadata.name, "name");
        assert_eq!(candid_token_metadata.symbol, "symbol");
        assert_eq!(candid_token_metadata.decimals, 1u8);
        assert_eq!(token_metadata.fee.minimum, BigUint::from(1u32));
        assert_eq!(token_metadata.fee.rate, 2);
        assert_eq!(token_metadata.fee.rate_decimals, 3);
    }
}
