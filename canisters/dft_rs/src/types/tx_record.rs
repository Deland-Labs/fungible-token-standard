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

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecordResult {
    // Return tx record if exist in the DFT cache txs
    Ok(TxRecord),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(String),
}


#[test]
fn test_tx_record_size() {
    let tx_record_size = std::mem::size_of::<TxRecord>();
    assert_eq!(
        176, tx_record_size,
        "tx_record_size is not {}",
        tx_record_size
    );
}
