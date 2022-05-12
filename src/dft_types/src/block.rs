use crate::{BlockHash, CandidTransaction, CommonResult, DFTError, Operation, Transaction};
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Block {
    #[serde(rename = "parentHash")]
    pub parent_hash: BlockHash,
    pub transaction: Transaction,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CandidBlock {
    #[serde(rename = "parentHash")]
    pub parent_hash: BlockHash,
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
        token_id: &Principal,
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
            token_id,
            parent_hash,
            transaction,
            timestamp,
        ))
    }

    pub fn new_from_transaction(
        token_id: &Principal,
        parent_hash: Option<BlockHash>,
        transaction: Transaction,
        timestamp: u64,
    ) -> Self {
        Self {
            parent_hash: parent_hash
                .unwrap_or_else(|| dft_utils::sha256::compute_hash(token_id.as_slice())),
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

    pub fn parent_hash(&self) -> BlockHash {
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
        let tx_bytes = bincode::serialize(&self).unwrap();
        let combine_bytes = [token_id.as_slice(), &tx_bytes[..]].concat();
        dft_utils::sha256::compute_hash(&combine_bytes)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CandidOperation, TokenHolder};
    use dft_utils::sha256::compute_hash;
    use std::convert::TryInto;

    #[test]
    fn test_block_size() {
        let block_size = std::mem::size_of::<Block>();
        let should_be_size = 184;
        assert_eq!(
            should_be_size, block_size,
            "Block size should be {} bytes, but is {} bytes",
            should_be_size, block_size
        );
    }

    #[test]
    fn test_block_encode_decode() {
        let token_id: Principal = "ryjl3-tyaaa-aaaaa-aaaba-cai".parse().unwrap();
        let caller: Principal = "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
            .parse()
            .unwrap();
        let new_fee_to: Principal =
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap();
        let parent_hash = compute_hash(token_id.as_slice());
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();
        let transaction = Transaction {
            operation: Operation::FeeToModify {
                caller: caller.clone(),
                new_fee_to: TokenHolder::new(new_fee_to.clone(), None),
            },
            created_at: now,
        };
        let block =
            Block::new_from_transaction(&token_id, Some(parent_hash), transaction.clone(), now);
        let encoded_block = block.clone().encode().unwrap();
        let decoded_block = encoded_block.clone().decode().unwrap();
        assert_eq!(block, decoded_block);

        assert_eq!(now, block.timestamp());

        let encoded_block_bytes = encoded_block.clone().into_vec();
        assert_eq!(encoded_block_bytes.len(), encoded_block.size_bytes());
        let decoded_block_bytes = decoded_block.encode().unwrap().into_vec();
        assert_eq!(encoded_block_bytes, decoded_block_bytes);

        let tx = block.transaction();
        let tx_hash = tx.hash_with_token_id(&token_id);
        assert_eq!(tx_hash, transaction.hash_with_token_id(&token_id));
    }

    #[test]
    fn test_block_to_candid_block() {
        let token_id: Principal = "ryjl3-tyaaa-aaaaa-aaaba-cai".parse().unwrap();
        let caller: Principal = "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
            .parse()
            .unwrap();
        let new_owner: Principal =
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap();
        let parent_hash = compute_hash(token_id.as_slice());
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();
        let transaction = Transaction {
            operation: Operation::OwnerModify {
                caller: caller.clone(),
                new_owner: new_owner.clone(),
            },
            created_at: now,
        };
        let block =
            Block::new_from_transaction(&token_id, Some(parent_hash), transaction.clone(), now);
        let candidate_block: CandidBlock = block.into();

        match candidate_block.transaction.operation {
            CandidOperation::OwnerModify { caller, new_owner } => {
                assert_eq!(caller, caller);
                assert_eq!(new_owner, new_owner);
            }
            _ => {}
        };
    }
}
