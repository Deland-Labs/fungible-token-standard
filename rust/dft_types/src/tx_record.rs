use super::{TokenHolder, TokenReceiver};
use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecord {
    // tx_index, caller, owner, spender, value, fee, timestamp
    Approve(Nat, Principal, TokenHolder, TokenReceiver, Nat, Nat, u64),
    // tx_index, caller, from, to, value, fee, timestamp
    Transfer(Nat, Principal, TokenHolder, TokenReceiver, Nat, Nat, u64),
    Forward(Principal),
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
