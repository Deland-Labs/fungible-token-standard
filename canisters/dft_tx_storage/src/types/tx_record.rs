use super::{TokenHolder, TokenReceiver};
use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecord {
    //  owner, spender, value, fee, timestamp
    Approve(TokenHolder, TokenReceiver, u128, u128, u64),
    // caller, from, to, value, fee, timestamp
    Transfer(TokenHolder, TokenReceiver, u128, u128, u64),
    // caller, from, value, timestamp
    Burn(TokenHolder, u128, u64),
}
