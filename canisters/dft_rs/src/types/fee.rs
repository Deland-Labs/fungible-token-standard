use ic_cdk::export::candid::{CandidType, Deserialize};

// Rate decimals = 8
// transferFee = cmp::max(lowest,amount * rate / 10^8)
#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct Fee {
    pub lowest: u128,
    pub rate: u128,
}
