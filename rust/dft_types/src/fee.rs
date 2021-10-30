use candid::{CandidType, Deserialize, Nat};

// Rate decimals = 8
// transferFee = cmp::max(minimum,amount * rate / 10^8)
#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct Fee {
    pub minimum: Nat,
    pub rate: Nat,
}
