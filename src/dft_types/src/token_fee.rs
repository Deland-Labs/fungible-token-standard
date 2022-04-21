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

impl TokenFee {
    pub fn new(minimum: TokenAmount, rate: u32, rate_decimals: u8) -> Self {
        TokenFee {
            minimum,
            rate,
            rate_decimals,
        }
    }

    pub fn calc_approve_fee(&self, _: &TokenAmount) -> TokenAmount {
        self.minimum.clone()
    }

    pub fn calc_transfer_fee(&self, amount: &TokenAmount) -> TokenAmount {
        let rate_fee = amount.clone() * self.rate / 10u128.pow(self.rate_decimals.into());
        self.minimum.clone().max(rate_fee)
    }
}

#[derive(CandidType, Default, Debug, Hash, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CandidTokenFee {
    pub minimum: Nat,
    pub rate: u32,
    #[serde(rename = "rateDecimals")]
    pub rate_decimals: u8,
}

impl From<TokenFee> for CandidTokenFee {
    fn from(fee: TokenFee) -> Self {
        CandidTokenFee {
            minimum: fee.minimum.into(),
            rate: fee.rate,
            rate_decimals: fee.rate_decimals,
        }
    }
}

impl From<CandidTokenFee> for TokenFee {
    fn from(fee: CandidTokenFee) -> Self {
        TokenFee {
            minimum: fee.minimum.into(),
            rate: fee.rate,
            rate_decimals: fee.rate_decimals,
        }
    }
}
