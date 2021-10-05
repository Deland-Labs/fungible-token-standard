#![allow(dead_code)]

use ic_cdk::export::Principal;
const CANISTER_ID_HASH_LEN_IN_BYTES: usize = 10;
const DFT_DOMAIN_SEPERATOR: &[u8] = b"\x0DFT-tx-id";
const MSG_INVALID_DFT_TX_ID: &str = "Invalid dft tx id";
//  length 45  [20: token_cansiter_id, 9: FUNGIBLE_TOKEN_CURSOR_SEPERATOR, 16: tx_cursor_id]
pub fn encode_tx_id(token_cansiter_id: Principal, tx_index: u128) -> String {
    let mut blob: Vec<u8> = Vec::new();
    let canister_id_blob = token_cansiter_id.as_slice();
    let tx_cursor_blob = u128_to_bytes(tx_index);
    blob.extend(DFT_DOMAIN_SEPERATOR);
    blob.extend(canister_id_blob);
    blob.extend(tx_cursor_blob);
    hex::encode(blob)
}

pub fn decode_tx_id(tx_id: String) -> Result<(Principal, u128), String> {
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

    if tx_cursor_blob.len() > 16 {
        return Err(MSG_INVALID_DFT_TX_ID.to_string());
    }

    let mut tx_index: [u8; 16] = Default::default();
    let fill_bytes: Vec<u8> = (0..(16 - tx_cursor_blob.len())).map(|_| 0u8).collect();

    tx_index.copy_from_slice([fill_bytes.as_slice(), tx_cursor_blob].concat().as_slice());

    Ok((canister_id_res.unwrap(), u128::from_be_bytes(tx_index)))
}

fn u128_to_bytes(n: u128) -> Vec<u8> {
    let bytes = n.to_be_bytes();

    for (k, &v) in bytes.iter().enumerate() {
        if v != 0 {
            return bytes[k..].to_vec();
        }
    }
    return vec![0];
}

#[test]
fn test_encode_decode() {
    let token_id = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
    let tx_index = 18446744073709552999u128;
    let tx_id = encode_tx_id(token_id, tx_index);

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
