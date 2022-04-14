use crate::{TokenFee, TokenHolder, TokenReceiver, TransactionHash};
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
    // It is not necessary to record these unimportant transactions
    // LogoModify {
    //     caller: Principal,
    //     #[serde(rename = "newLogo")]
    //     new_logo: Vec<u8>,
    // },
    // DescModify {
    //     caller: Principal,
    //     #[serde(rename = "newDesc")]
    //     new_desc: Vec<(String, String)>,
    // },
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransactionInfo {
    pub block_timestamp: u64,
    pub tx_hash: TransactionHash,
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
