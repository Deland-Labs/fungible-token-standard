use crate::{BlockHash, Operation, Transaction};
use candid::{CandidType, Deserialize};
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use serde::Serialize;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Block {
    #[serde(rename = "parentHash")]
    pub parent_hash: Option<BlockHash>,
    pub transaction: Transaction,
    pub timestamp: u64,
}

impl Block {
    pub fn new(
        parent_hash: Option<BlockHash>,
        operation: Operation,
        created_at: u64, // transaction timestamp
        timestamp: u64,  // block timestamp
    ) -> Result<Self, String> {
        let transaction = Transaction {
            operation,
            created_at,
        };
        Ok(Self::new_from_transaction(
            parent_hash,
            transaction,
            timestamp,
        ))
    }

    pub fn new_from_transaction(
        parent_hash: Option<BlockHash>,
        transaction: Transaction,
        timestamp: u64,
    ) -> Self {
        Self {
            parent_hash,
            transaction,
            timestamp,
        }
    }

    pub fn encode(self) -> Result<EncodedBlock, String> {
        let bytes = candid::encode_one(&self);
        match bytes {
            Ok(b) => Ok(EncodedBlock::from(b)),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn parent_hash(&self) -> Option<BlockHash> {
        self.parent_hash.clone()
    }

    pub fn transaction(&self) -> Cow<Transaction> {
        Cow::Borrowed(&self.transaction)
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodedBlock(pub serde_bytes::ByteBuf);

impl From<Vec<u8>> for EncodedBlock {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_vec(bytes)
    }
}

impl EncodedBlock {
    pub fn hash(&self) -> BlockHash {
        let mut sha = Sha256::new();
        sha.update(&self.0);
        sha.finalize().into()
    }

    pub fn decode(&self) -> Result<Block, String> {
        let bytes = self.0.to_vec();
        let block = candid::decode_one::<Block>(&bytes);
        match block {
            Ok(b) => Ok(b),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self(serde_bytes::ByteBuf::from(bytes))
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn size_bytes(&self) -> usize {
        self.0.len()
    }
}

