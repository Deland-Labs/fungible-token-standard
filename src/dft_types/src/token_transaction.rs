use crate::{TokenAmount, TokenFee, TokenHolder, TokenReceiver, TransactionHash, token_fee::CandidTokenFee};
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize, Serialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Approve {
        caller: Principal,
        owner: TokenHolder,
        spender: TokenHolder,
        value: TokenAmount,
        fee: TokenAmount,
    },
    Transfer {
        caller: TokenHolder,
        from: TokenHolder,
        to: TokenReceiver,
        value: TokenAmount,
        fee: TokenAmount,
    },
    FeeModify {
        caller: Principal,
        #[serde(rename = "newFee")]
        new_fee: TokenFee,
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
}

#[derive(CandidType, Deserialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CandidOperation {
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
        #[serde(rename = "newFee")]
        new_fee: CandidTokenFee,
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
}

impl From<Operation> for CandidOperation
{
    fn from(operation: Operation) -> Self {
        match operation {
            Operation::Approve {
                caller,
                owner,
                spender,
                value,
                fee,
            } => CandidOperation::Approve {
                caller,
                owner,
                spender,
                value: value.into(),
                fee: fee.into(),
            },
            Operation::Transfer {
                caller,
                from,
                to,
                value,
                fee,
            } => CandidOperation::Transfer {
                caller,
                from,
                to,
                value: value.into(),
                fee: fee.into(),
            },
            Operation::FeeModify { caller, new_fee } => CandidOperation::FeeModify {
                caller,
                new_fee: new_fee.into(),
            },
            Operation::OwnerModify { caller, new_owner } => CandidOperation::OwnerModify {
                caller,
                new_owner,
            },
            Operation::FeeToModify { caller, new_fee_to } => CandidOperation::FeeToModify {
                caller,
                new_fee_to,
            },
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TransactionInfo {
    pub block_timestamp: u64,
    pub tx_hash: TransactionHash,
}

#[derive(Deserialize, Serialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    pub operation: Operation,
    pub created_at: u64,
}

impl Transaction {
    // hash token id + tx bytes, make sure tx hash unique
    pub fn hash_with_token_id(&self, token_id: &Principal) -> TransactionHash {
        let mut sha = Sha256::new();
        let tx_bytes = bincode::serialize(&self).unwrap();
        let combine_bytes = [token_id.as_slice(), &tx_bytes[..]].concat();
        sha.update(combine_bytes);
        sha.finalize().into()
    }
}

#[derive(CandidType, Deserialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CandidTransaction {
    pub operation: CandidOperation,
    /// The time this transaction was created.
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

impl From<Transaction> for CandidTransaction
{
    fn from(tx: Transaction) -> Self {
        CandidTransaction {
            operation: CandidOperation::from(tx.operation),
            created_at: tx.created_at,
        }
    }
}
