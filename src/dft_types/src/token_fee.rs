use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use crate::TokenAmount;

// rate decimals = 8
// transferFee = cmp::max(minimum,amount * rate / 10^8)
#[derive(Default, Debug, Hash, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct InnerTokenFee {
    pub minimum: TokenAmount,
    pub rate: u32,
    #[serde(rename = "rateDecimals")]
    pub rate_decimals: u8,
}

impl InnerTokenFee {
    pub fn new(minimum: TokenAmount, rate: u32, rate_decimals: u8) -> Self {
        InnerTokenFee {
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
pub struct TokenFee {
    pub minimum: Nat,
    pub rate: u32,
    #[serde(rename = "rateDecimals")]
    pub rate_decimals: u8
}

impl From<InnerTokenFee> for TokenFee {
    fn from(fee: InnerTokenFee) -> Self {
        TokenFee {
            minimum: fee.minimum.into(),
            rate: fee.rate,
            rate_decimals: fee.rate_decimals,
        }
    }
}

impl From<TokenFee> for InnerTokenFee {
    fn from(fee: TokenFee) -> Self {
        InnerTokenFee {
            minimum: fee.minimum.into(),
            rate: fee.rate,
            rate_decimals: fee.rate_decimals,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_fee_calc_approve_fee() {
        let fee = InnerTokenFee::new(1u32.into(), 1, 8);
        let amount = TokenAmount::from(1000u32);
        assert_eq!(fee.calc_approve_fee(&amount), 1u32.into());

        let fee = InnerTokenFee::new(1u32.into(), 1, 2);
        let amount = TokenAmount::from(1000u32);
        assert_eq!(fee.calc_approve_fee(&amount), 1u32.into());
    }

    #[test]
    fn test_token_fee_calc_transfer_fee() {
        let fee = InnerTokenFee::new(1u32.into(), 1, 8);
        let amount = TokenAmount::from(1000u32);
        assert_eq!(fee.calc_transfer_fee(&amount), 1u32.into());

        let fee = InnerTokenFee::new(1u32.into(), 1, 2);
        let amount = TokenAmount::from(1000u32);
        assert_eq!(fee.calc_transfer_fee(&amount), 10u32.into());
    }

    #[test]
    fn test_to_candid_token_fee() {
        let fee = InnerTokenFee::new(1u32.into(), 1, 8);
        let candid_fee = TokenFee::from(fee);
        assert_eq!(candid_fee.minimum, Nat::from(1u32));
        assert_eq!(candid_fee.rate, 1);
        assert_eq!(candid_fee.rate_decimals, 8);
    }
    #[test]
    fn test_from_candid_fee_to_token_fee() {
        let fee = TokenFee {
            minimum: 1u32.into(),
            rate: 1u32,
            rate_decimals: 8u8,
        };
        let token_fee = InnerTokenFee::from(fee.clone());
        assert_eq!(token_fee.minimum, fee.minimum.0);
        assert_eq!(token_fee.rate, fee.rate);
        assert_eq!(token_fee.rate_decimals, fee.rate_decimals);
    }
}
