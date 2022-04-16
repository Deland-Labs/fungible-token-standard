use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use crate::TokenAmount;

// rate decimals = 8
// transferFee = cmp::max(minimum,amount * rate / 10^8)
#[derive(Default, Debug, Hash, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenFee {
    pub minimum: TokenAmount,
    pub rate: u32,
    #[serde(rename = "rateDecimals")]
    pub rate_decimals: u8,
}

#[derive(CandidType, Default, Debug, Hash, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CandidTokenFee {
    pub minimum: Nat,
    pub rate: u32,
    #[serde(rename = "rateDecimals")]
    pub rate_decimals: u8,
}

impl From<TokenFee> for CandidTokenFee
{
    fn from(fee: TokenFee) -> Self {
        CandidTokenFee {
            minimum: fee.minimum.into(),
            rate: fee.rate,
            rate_decimals: fee.rate_decimals,
        }
    }
}

impl From<CandidTokenFee> for TokenFee{
    fn from(fee: CandidTokenFee) -> Self {
        TokenFee {
            minimum: fee.minimum.into(),
            rate: fee.rate,
            rate_decimals: fee.rate_decimals,
        }
    }
}