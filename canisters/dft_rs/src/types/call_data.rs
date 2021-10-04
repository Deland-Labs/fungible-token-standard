use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct CallData {
    pub method: String,
    pub args: Vec<u8>,
}
