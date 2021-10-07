use candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub struct KeyValuePair {
    pub k: String,
    pub v: String,
}
