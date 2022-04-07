use candid::{CandidType, Deserialize, Nat};

// rate decimals = 8
// transferFee = cmp::max(minimum,amount * rate / 10^8)
#[derive(CandidType, Default, Debug, Hash, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fee {
    pub minimum: Nat,
    pub rate: Nat,
    #[serde(rename = "rateDecimals")]
    pub rate_decimals: u8,
}
