use candid::{Nat, Principal};
use dft_types::{BlockHeight, TokenAmount, TokenHolder};
use dft_utils::principal::is_canister;
use log::{debug, info, warn};

pub trait ITransferNotifyAPI {
    fn notify(&self, receiver: &String, block_height: &BlockHeight, transfer_from: &TokenHolder, transfer_value: &TokenAmount);
}

#[derive(Default)]
pub struct TransferNotifyAPI;

#[cfg_attr(coverage_nightly, no_coverage)]
impl ITransferNotifyAPI for TransferNotifyAPI {
    fn notify(&self, receiver: &String, block_height: &BlockHeight, transfer_from: &TokenHolder, transfer_value: &TokenAmount) {
        let pid = Principal::from_text(receiver);
        debug!("TransferNotifyAPI::notify in");
        if let Ok(receiver_canister_id) = pid {
            if is_canister(&receiver_canister_id) {
                info!("TransferNotifyAPI::notify:  will notify receiver: {:?},height is {},transfer_from is {},transfer_value is {}",
                    receiver_canister_id.to_text(),block_height,transfer_from.clone().to_hex(),transfer_value.clone());
                //notify receiver
                let nat_block_height: Nat = block_height.clone().into();
                let nat_transfer_value: Nat = transfer_value.clone().into();
                ic_cdk::notify(
                    receiver_canister_id.clone(),
                    "onTokenReceived",
                    (nat_block_height, transfer_from.clone(), nat_transfer_value, ),
                )
                    .unwrap_or_else(|reject| {
                        warn!(
                        "failed to notify (receiver_canister_id={}): {:?}",
                        receiver_canister_id, reject
                    )
                    });
            }
        };
    }
}
