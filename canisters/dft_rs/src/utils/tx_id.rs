#![allow(dead_code)]

use candid::Nat;
use ic_cdk::export::Principal;
use num_bigint::BigUint;
const CANISTER_ID_HASH_LEN_IN_BYTES: usize = 10;
const DFT_DOMAIN_SEPERATOR: &[u8] = b"\x0DFT-tx-id";
const MSG_INVALID_DFT_TX_ID: &str = "Invalid dft tx id";
//  length 45  [20: token_cansiter_id, 9: FUNGIBLE_TOKEN_CURSOR_SEPERATOR, 16: tx_cursor_id]
pub fn encode_tx_id(token_cansiter_id: Principal, tx_index: Nat) -> String {
    let mut blob: Vec<u8> = Vec::new();
    let canister_id_blob = token_cansiter_id.as_slice();
    let tx_cursor_blob = tx_index.0.to_bytes_be();
    blob.extend(DFT_DOMAIN_SEPERATOR);
    blob.extend(canister_id_blob);
    blob.extend(tx_cursor_blob);
    hex::encode(blob)
}

pub fn decode_tx_id(tx_id: String) -> Result<(Principal, Nat), String> {
    let blob = hex::decode(tx_id).unwrap();

    let dft_domain_seperator_blob = &blob[0..DFT_DOMAIN_SEPERATOR.len()];
    let canister_id_end_index = CANISTER_ID_HASH_LEN_IN_BYTES + DFT_DOMAIN_SEPERATOR.len();
    let canister_id_blob = &blob[DFT_DOMAIN_SEPERATOR.len()..canister_id_end_index];
    let tx_cursor_blob = &blob[canister_id_end_index..];
    let canister_id_res = Principal::try_from_slice(canister_id_blob);
    match canister_id_res {
        Err(_) => return Err(MSG_INVALID_DFT_TX_ID.to_string()),
        _ => {}
    };

    if dft_domain_seperator_blob != DFT_DOMAIN_SEPERATOR {
        return Err(MSG_INVALID_DFT_TX_ID.to_string());
    }

    Ok((
        canister_id_res.unwrap(),
        BigUint::from_bytes_be(tx_cursor_blob).into(),
    ))
}

#[test]
fn test_encode_decode() {
    let token_id = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
    let tx_index = Nat::from(18446744073709552999u128);
    let tx_id = encode_tx_id(token_id, tx_index.clone());

    let tx_id_decode_res = decode_tx_id(tx_id);

    match tx_id_decode_res {
        Ok((token_id_de, tx_index_de)) => {
            assert_eq!(token_id_de, token_id, "token id check failed");
            assert_eq!(tx_index, tx_index_de, "tx index check failed");
        }
        Err(msg) => {
            assert!(false, "failed with {}", msg);
        }
    };
}
