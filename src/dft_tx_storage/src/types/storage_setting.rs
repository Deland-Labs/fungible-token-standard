use candid::{Deserialize, Principal};
use dft_types::{CommonResult, DFTError, StableState};
use getset::{Getters, Setters};
use num_bigint::BigUint;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, Getters, Setters)]
#[getset(get = "pub")]
pub struct StorageSetting {
    token_id: Principal,
    block_height_offset: BigUint,
    create_at: u64,
}

impl Default for StorageSetting {
    fn default() -> Self {
        StorageSetting {
            token_id: Principal::anonymous(),
            block_height_offset: 0u8.into(),
            create_at: 0,
        }
    }
}

impl StorageSetting {
    pub fn initialize(&mut self, token_id: Principal, block_height_offset: BigUint, now: u64) {
        assert!(self.token_id == Principal::anonymous() && self.create_at == 0);
        self.token_id = token_id;
        self.block_height_offset = block_height_offset;
        self.create_at = now;
    }
    // fn only allow token canister
    pub fn only_allow_token_canister(&self, caller: &Principal) -> CommonResult<()> {
        if &self.token_id != caller {
            return Err(DFTError::OnlyAllowTokenCanisterCallThisFunction);
        }
        Ok(())
    }
}

impl StableState for StorageSetting {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&(
            self.token_id,
            self.block_height_offset.clone(),
            self.create_at,
        ))
        .unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let (token_id, block_height_offset, create_at): (Principal, BigUint, u64) =
            bincode::deserialize(&bytes).unwrap();

        Ok(StorageSetting {
            token_id,
            block_height_offset,
            create_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use std::convert::TryInto;

    #[test]
    fn test_storage_setting_encode_decode() {
        let test_token_id: Principal = "rwlgt-iiaaa-aaaaa-aaaaa-cai".parse().unwrap();
        let block_height_offset = BigUint::from(1u8);
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        let mut storage_setting = StorageSetting::default();
        storage_setting.initialize(test_token_id.clone(), block_height_offset.clone(), now);
        let encoded = storage_setting.encode();
        let decoded = StorageSetting::decode(encoded).unwrap();

        assert_eq!(storage_setting.token_id, decoded.token_id);
        assert_eq!(
            storage_setting.block_height_offset,
            decoded.block_height_offset
        );
        assert_eq!(storage_setting.create_at, decoded.create_at);
    }

    #[test]
    fn test_storage_setting_only_allow_token_canister() {
        let test_token_id: Principal = "rwlgt-iiaaa-aaaaa-aaaaa-cai".parse().unwrap();
        let block_height_offset = BigUint::from(1u8);
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        let caller: Principal = "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
            .parse()
            .unwrap();

        let storage_setting = StorageSetting {
            token_id: test_token_id,
            block_height_offset,
            create_at: now,
        };

        assert!(storage_setting
            .only_allow_token_canister(&test_token_id)
            .is_ok());
        assert!(storage_setting.only_allow_token_canister(&caller).is_err());
    }
}
