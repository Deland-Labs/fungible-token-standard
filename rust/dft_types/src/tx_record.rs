use super::{TokenHolder, TokenReceiver};
use crate::{ActorError, DFTError, Fee};
use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Debug, Clone, Deserialize, Eq, PartialEq)]
pub enum TxRecord {
    // tx_index, caller, owner, spender, value, fee,nonce , timestamp
    Approve(
        Nat,
        TokenHolder,
        TokenHolder,
        TokenReceiver,
        Nat,
        Nat,
        u64,
        u64,
    ),
    // tx_index, caller, from, to, value, fee, nonce, timestamp
    Transfer(
        Nat,
        TokenHolder,
        TokenHolder,
        TokenReceiver,
        Nat,
        Nat,
        u64,
        u64,
    ),
    // tx_index, caller (owner), new fee setting, nonce, timestamp
    FeeModify(
        Nat,
        Principal,
        Fee,
        u64,
        u64,
    ),
     // tx_index, caller (owner), new owner, nonce, timestamp
     OwnerModify(
        Nat,
        Principal,
        Principal,
        u64,
        u64,
    ),
    // tx_index, caller (owner), new feeTo, nonce, timestamp
    FeeToModify(
        Nat,
        Principal,
        TokenHolder,
        u64,
        u64,
    ),
}

impl TxRecord {
    pub fn get_tx_index(&self) -> Nat {
        match self {
            TxRecord::Approve(tx_index, _, _, _, _, _, _, _) => tx_index.clone(),
            TxRecord::Transfer(tx_index, _, _, _, _, _, _, _) => tx_index.clone(),
            TxRecord::FeeModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::OwnerModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::FeeToModify(tx_index, _, _, _, _) => tx_index.clone(),    
        }
    }
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecordCommonResult {
    // Return tx record if exist in the DFT cache txs
    Ok(TxRecord),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(DFTError),
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecordResult {
    // Return tx record if exist in the DFT cache txs
    Ok(TxRecord),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(ActorError),
}

impl From<TxRecordCommonResult> for TxRecordResult {
    fn from(r: TxRecordCommonResult) -> Self {
        match r {
            TxRecordCommonResult::Ok(tx) => TxRecordResult::Ok(tx),
            TxRecordCommonResult::Forward(p) => TxRecordResult::Forward(p),
            TxRecordCommonResult::Err(e) => TxRecordResult::Err(e.into()),
        }
    }
}

#[test]
fn test_tx_record_size() {
    let tx_record_size = std::mem::size_of::<TxRecord>();
    assert_eq!(
        184, tx_record_size,
        "tx_record_size is not {}",
        tx_record_size
    );
}
