use crate::{BlockHash, CandidTransaction, CommonResult, DFTError, Operation, Transaction};
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Block {
    #[serde(rename = "parentHash")]
    pub parent_hash: Option<BlockHash>,
    pub transaction: Transaction,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CandidBlock {
    #[serde(rename = "parentHash")]
    pub parent_hash: Option<BlockHash>,
    pub transaction: CandidTransaction,
    pub timestamp: u64,
}

impl From<Block> for CandidBlock {
    fn from(block: Block) -> Self {
        CandidBlock {
            parent_hash: block.parent_hash,
            transaction: block.transaction.into(),
            timestamp: block.timestamp,
        }
    }
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

    pub fn encode(self) -> CommonResult<EncodedBlock> {
        let bytes = bincode::serialize(&self);
        match bytes {
            Ok(b) => Ok(EncodedBlock::from(b)),
            Err(e) => Err(DFTError::Unknown {
                detail: format!("block encode failed,{}", e),
            }),
        }
    }

    pub fn parent_hash(&self) -> Option<BlockHash> {
        self.parent_hash
    }

    pub fn transaction(&self) -> Cow<Transaction> {
        Cow::Borrowed(&self.transaction)
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct EncodedBlock(pub serde_bytes::ByteBuf);

impl From<Vec<u8>> for EncodedBlock {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_vec(bytes)
    }
}

impl EncodedBlock {
    // hash token id + block bytes, ensuring that the block hash of different tokens is unique.
    pub fn hash_with_token_id(&self, token_id: &Principal) -> BlockHash {
        let mut sha = Sha256::new();
        let tx_bytes = bincode::serialize(&self).unwrap();
        let combine_bytes = [token_id.as_slice(), &tx_bytes[..]].concat();
        sha.update(combine_bytes);
        sha.finalize().into()
    }

    pub fn decode(&self) -> CommonResult<Block> {
        let bytes = self.0.to_vec();
        let block = bincode::deserialize::<Block>(&bytes[..]);
        match block {
            Ok(b) => Ok(b),
            Err(e) => Err(DFTError::Unknown {
                detail: format!("decode block failed,{}", e),
            }),
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

#[test]
fn test_block_size() {
    let block_size = std::mem::size_of::<Block>();
    let should_be_size = 192;
    assert_eq!(
        should_be_size, block_size,
        "Block size should be {} bytes, but is {} bytes",
        should_be_size, block_size
    );
}
