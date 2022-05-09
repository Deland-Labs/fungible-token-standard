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

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_is_canister() {
        let canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let principal_id =
            Principal::from_text("zctcl-skicc-vgye3-xaoas-gt6d4-o2a54-v6era-nw6hm-bda6p-ur54p-oae")
                .unwrap();
        assert!(is_canister(&canister_id));
        assert!(!is_canister(&principal_id));
    }

    #[test]
    fn test_is_user_principal() {
        let canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let principal_id =
            Principal::from_text("zctcl-skicc-vgye3-xaoas-gt6d4-o2a54-v6era-nw6hm-bda6p-ur54p-oae")
                .unwrap();
        assert!(!is_user_principal(&canister_id));
        assert!(is_user_principal(&principal_id));
    }
}
