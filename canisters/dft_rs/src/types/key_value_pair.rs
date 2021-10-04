use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePair {
    pub k: String,
    pub v: String,
}
