mod account_identifier;
mod token_holder;
mod tx_record;
mod storage_payload;

pub type Txs = Vec<TxRecord>;
pub use account_identifier::AccountIdentifier;
pub use account_identifier::Subaccount;
pub use account_identifier::SUB_ACCOUNT_ZERO;
pub use token_holder::TokenHolder;
pub type TokenReceiver = TokenHolder;
pub use tx_record::TxRecord;
pub use storage_payload::StoragePayload;
