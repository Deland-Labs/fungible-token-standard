use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct StatisticsInfo {
    pub holders: u128,
    pub transfers: u128,
}
