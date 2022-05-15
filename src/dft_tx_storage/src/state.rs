use crate::types::*;
use dft_types::*;
use ic_cdk::api::stable::{stable_bytes, StableWriter};
use ic_cdk_macros::*;
use log::{error, info};
use std::cell::RefCell;

thread_local! {
      pub static STATE : State = State::default();
}
#[derive(Default, Debug)]
pub struct State {
    pub storage_setting: RefCell<StorageSetting>,
    pub block_archive: RefCell<BlockArchive>,
}

impl State {
    pub fn replace(&self, new_state: State) {
        self.storage_setting
            .replace(new_state.storage_setting.take());
        self.block_archive.replace(new_state.block_archive.take());
    }
}

impl StableState for State {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&(
            self.storage_setting.borrow().encode(),
            self.block_archive.borrow().encode(),
        ))
        .unwrap()
    }

    #[allow(clippy::type_complexity)]
    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let (storage_setting_bytes, block_archive_bytes): (Vec<u8>, Vec<u8>) =
            bincode::deserialize(&bytes).unwrap();

        Ok(State {
            storage_setting: RefCell::new(StorageSetting::decode(storage_setting_bytes)?),
            block_archive: RefCell::new(BlockArchive::decode(block_archive_bytes)?),
        })
    }
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|s| {
        let bytes = s.encode();
        match StableWriter::default().write(bytes.as_slice()) {
            Ok(size) => {
                info!(
                    "auto-scaling-storage: after pre_upgrade stable_write size{}",
                    size
                );
            }
            Err(_) => {
                error!("auto-scaling-storage: {}", "stable_write error");
            }
        }
    })
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| {
        let bytes = stable_bytes();
        let restore_state =
            State::decode(bytes).expect("auto-scaling-storage: Decoding stable memory failed");
        s.replace(restore_state);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;
    use num_bigint::BigUint;
    use std::convert::TryInto;

    #[test]
    fn test_state_encode_decode() {
        let test_token_id: Principal = "rwlgt-iiaaa-aaaaa-aaaaa-cai".parse().unwrap();
        let block_height_offset = BigUint::from(1u8);
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        let state = State::default();
        state.storage_setting.borrow_mut().initialize(
            test_token_id.clone(),
            block_height_offset.clone(),
            now,
        );

        let bytes = state.encode();
        let restore_state =
            State::decode(bytes).expect("auto-scaling-storage: Decoding stable memory failed");

        let setting2 = state.storage_setting.borrow_mut();
        let restore_setting = restore_state.storage_setting.borrow();
        assert_eq!(setting2.token_id(), restore_setting.token_id());
        assert_eq!(
            setting2.block_height_offset(),
            restore_setting.block_height_offset()
        );
        assert_eq!(setting2.create_at(), restore_setting.create_at());
    }
}
