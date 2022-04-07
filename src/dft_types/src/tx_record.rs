use super::{TokenHolder, TokenReceiver};
use crate::{DFTError, TokenFee, TransactionHash};
use candid::{CandidType, Deserialize, Nat, Principal};
use sha2::{Digest, Sha256};

#[derive(Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Approve {
        caller: Principal,
        owner: TokenHolder,
        spender: TokenHolder,
        value: Nat,
        fee: Nat,
    },
    Transfer {
        caller: TokenHolder,
        from: TokenHolder,
        to: TokenReceiver,
        value: Nat,
        fee: Nat,
    },
    FeeModify {
        caller: Principal,
        fee: TokenFee,
    },
    OwnerModify {
        caller: Principal,
        #[serde(rename = "newOwner")]
        new_owner: Principal,
    },
    FeeToModify {
        caller: Principal,
        #[serde(rename = "newFeeTo")]
        new_fee_to: TokenHolder,
    },
    LogoModify {
        caller: Principal,
        #[serde(rename = "newLogo")]
        new_logo: Vec<u8>,
    },
    DescModify {
        caller: Principal,
        #[serde(rename = "newDesc")]
        new_desc: Vec<(String, String)>,
    },
}

#[derive(Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    pub operation: Operation,
    /// The time this transaction was created.
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

impl Transaction {
    // hash token id + tx bytes, make sure tx hash unique
    pub fn hash_with_token_id(&self, token_id: &Principal) -> TransactionHash {
        let mut sha = Sha256::new();
        let tx_bytes = candid::encode_one(&self).unwrap();
        let combine_bytes = [token_id.as_slice(), &tx_bytes[..]].concat();
        sha.update(combine_bytes);
        sha.finalize().into()
    }
}
#[derive(CandidType, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TxRecord {
    // tx_index, caller, owner, spender, value, fee,created_at , timestamp
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
    // tx_index, caller, from, to, value, fee, created_at, timestamp
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
    // tx_index, caller (owner), new fee setting, created_at, timestamp
    FeeModify(Nat, Principal, TokenFee, u64, u64),
    // tx_index, caller (owner), new owner, created_at, timestamp
    OwnerModify(Nat, Principal, Principal, u64, u64),
    // tx_index, caller (owner), new feeTo, created_at, timestamp
    FeeToModify(Nat, Principal, TokenHolder, u64, u64),
    // tx_index, caller (owner), new logo, created_at, timestamp
    LogoModify(Nat, Principal, Vec<u8>, u64, u64),
    DescModify(Nat, Principal, Vec<(String, String)>, u64, u64),
}

impl TxRecord {
    pub fn get_tx_index(&self) -> Nat {
        match self {
            TxRecord::Approve(tx_index, _, _, _, _, _, _, _) => tx_index.clone(),
            TxRecord::Transfer(tx_index, _, _, _, _, _, _, _) => tx_index.clone(),
            TxRecord::FeeModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::OwnerModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::FeeToModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::LogoModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::DescModify(tx_index, _, _, _, _) => tx_index.clone(),
        }
    }
    pub fn get_caller(&self) -> TokenHolder {
        match self {
            TxRecord::Approve(_, caller, _, _, _, _, _, _) => caller.clone(),
            TxRecord::Transfer(_, caller, _, _, _, _, _, _) => caller.clone(),
            TxRecord::FeeModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
            TxRecord::OwnerModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
            TxRecord::FeeToModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
            TxRecord::LogoModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
            TxRecord::DescModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
        }
    }

    pub fn get_created_at(&self) -> u64 {
        match self {
            TxRecord::Approve(_, _, _, _, _, _, created_at, _) => created_at.clone(),
            TxRecord::Transfer(_, _, _, _, _, _, created_at, _) => created_at.clone(),
            TxRecord::FeeModify(_, _, _, created_at, _) => created_at.clone(),
            TxRecord::OwnerModify(_, _, _, created_at, _) => created_at.clone(),
            TxRecord::FeeToModify(_, _, _, created_at, _) => created_at.clone(),
            TxRecord::LogoModify(_, _, _, created_at, _) => created_at.clone(),
            TxRecord::DescModify(_, _, _, created_at, _) => created_at.clone(),
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

#[test]
fn test_tx_record_size() {
    let tx_record_size = std::mem::size_of::<TxRecord>();
    assert_eq!(
        184, tx_record_size,
        "tx_record_size is not {}",
        tx_record_size
    );
}
