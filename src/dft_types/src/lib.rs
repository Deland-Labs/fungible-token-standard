mod account_identifier;
mod actor_response;
mod block;
mod blockchain;
pub mod constants;
mod errors;
mod http;
mod token_allowances;
mod token_archive;
mod token_balances;
mod token_description;
mod token_fee;
mod token_info;
mod token_metadata;
mod token_metrics;
mod token_transaction;

pub use account_identifier::*;
pub use actor_response::*;
pub use block::*;
pub use blockchain::*;
use candid::Nat;
use candid::Principal;
pub use errors::*;
pub use http::*;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::string::String;
pub use token_allowances::TokenAllowances;
pub use token_archive::*;
pub use token_balances::TokenBalances;
pub use token_description::TokenDescription;
pub use token_fee::*;
pub use token_info::TokenInfo;
pub use token_metadata::*;
pub use token_metrics::TokenMetrics;
pub use token_transaction::*;

pub type TokenAmount = BigUint;
pub type TransactionId = String;
pub type ExtendData = HashMap<String, String>;
pub type Balances = HashMap<TokenHolder, Nat>;
pub type StorageCanisterIds = HashMap<Nat, Principal>;
pub type Allowances = HashMap<TokenHolder, HashMap<TokenHolder, Nat>>;
pub type TokenHolder = AccountIdentifier;
pub type TransferFrom = TokenHolder;
pub type TokenReceiver = TokenHolder;
pub type BlockHash = [u8; 32];
pub type BlockHeight = BigUint;
pub type TransactionHash = [u8; 32];

#[test]
fn test_nat_size() {
    let nat_size = std::mem::size_of::<Nat>();
    assert_eq!(24, nat_size, "nat_size is not {}", 24);
}
