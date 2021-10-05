use super::{TokenHolder, TokenReceiver};
use ic_cdk::export::candid::{CandidType, Deserialize, Nat};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecord {
    //  owner, spender, value, fee, timestamp
    Approve(TokenHolder, TokenReceiver, Nat, Nat, u64),
    // caller, from, to, value, fee, timestamp
    Transfer(TokenHolder, TokenReceiver, Nat, Nat, u64),
    // caller, from, value, timestamp
    Burn(TokenHolder, Nat, u64),
}
