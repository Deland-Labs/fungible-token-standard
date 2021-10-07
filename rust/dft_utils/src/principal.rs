#![allow(dead_code)]
use candid::Principal;

const CANISTER_ID_HASH_LEN_IN_BYTES: usize = 10;
pub fn is_canister(id: &Principal) -> bool {
    let blob = id.as_slice();
    blob.len() == CANISTER_ID_HASH_LEN_IN_BYTES
}
const HASH_LEN_IN_BYTES: usize = 28;
const TYPE_SELF_AUTH: u8 = 0x02;
pub fn is_user_principal(id: &Principal) -> bool {
    let blob = id.as_slice();
    if blob.len() != HASH_LEN_IN_BYTES + 1 {
        return false;
    }
    if blob.last() != Some(&TYPE_SELF_AUTH) {
        return false;
    }
    true
}
