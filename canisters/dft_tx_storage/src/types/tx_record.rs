use super::{TokenHolder, TokenReceiver};
use candid::{Nat, Principal};
use ic_cdk::export::candid::{CandidType, Deserialize};
#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecord {
    // tx_index, caller, owner, spender, value, fee, timestamp
    Approve(Nat, Principal, TokenHolder, TokenReceiver, Nat, Nat, u64),
    // tx_index, caller, from, to, value, fee, timestamp
    Transfer(Nat, Principal, TokenHolder, TokenReceiver, Nat, Nat, u64),
    // tx_index, caller, from, value, timestamp
    Burn(Nat, Principal, TokenHolder, Nat, u64),
}
