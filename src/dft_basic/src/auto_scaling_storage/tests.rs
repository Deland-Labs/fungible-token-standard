use super::AutoScalingStorageService;
use crate::canister_api::*;
use crate::service::{basic_service, blockchain_service, management_service};
use async_trait::async_trait;
use candid::Principal;
use dft_types::constants::{
    DEFAULT_FEE_RATE_DECIMALS, MAX_CANISTER_STORAGE_BYTES, MIN_CANISTER_STORAGE_BYTES,
};
use dft_types::*;
use mockall::mock;
use num_bigint::BigUint;
use rstest::*;
use std::collections::VecDeque;
use std::sync::Arc;

#[fixture]
fn test_owner() -> Principal {
    Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae").unwrap()
}

// other caller
#[fixture]
fn other_caller() -> Principal {
    Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe").unwrap()
}

// minter
#[fixture]
fn test_minter() -> Principal {
    Principal::from_text("o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae").unwrap()
}

// spender
#[fixture]
fn test_spender() -> Principal {
    Principal::from_text("7zap4-dnqjf-k2oei-jj2uj-sw6db-eksrj-kzc5h-nmki4-x5fcn-w53an-gae").unwrap()
}

#[fixture]
fn test_fee_to() -> Principal {
    Principal::from_text("7b6mv-nyoey-gkj2b-2r6mp-fa2rr-6ktwc-qrx7e-l3eax-32jd7-ahwnj-3qe").unwrap()
}

#[fixture]
fn test_token_id() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}

#[fixture]
fn test_auto_scaling_storage_id() -> Principal {
    Principal::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap()
}

#[fixture]
fn test_auto_scaling_storage_id2() -> Principal {
    Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap()
}

#[fixture]
fn test_name() -> String {
    "Deland Labs Token".to_string()
}

#[fixture]
fn test_symbol() -> String {
    "DLT".to_string()
}

#[fixture]
fn test_decimals() -> u8 {
    18u8
}

// test fee 0 rate
#[fixture]
fn test_fee() -> TokenFee {
    TokenFee {
        minimum: 2u32.into(),
        rate: 0,
        rate_decimals: DEFAULT_FEE_RATE_DECIMALS,
    }
}

#[fixture]
fn test_token() {
    let fee_to = TokenHolder::new(test_fee_to(), None);
    basic_service::token_initialize(
        &test_owner(),
        test_token_id(),
        None,
        test_name(),
        test_symbol(),
        test_decimals(),
        test_fee(),
        fee_to,
        None,
    );
}

#[fixture]
fn now() -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    now as u64
}

#[fixture]
pub fn init_test() {
    dft_utils::ic_logger::init_test_logger();
}

mock! {
    pub ICManagementAPI {
    }
    #[async_trait]
    impl IICManagementAPI for ICManagementAPI {
        async fn create_canister(&self, args: CreateCanisterArgs) -> Result<CanisterIdRecord, String>;
        async fn canister_status(&self, id_record: CanisterIdRecord) -> Result<CanisterStatusResponse, String>;
        async fn canister_install(&self, canister_id: &Principal, wasm_module: Vec<u8>, args: Vec<u8>) -> Result<(), String>;
    }
}

#[fixture]
pub fn mock_ic_management_api() -> MockICManagementAPI {
    MockICManagementAPI::new()
}

mock! {
    pub DFTTxStorageAPI {
    }
    #[async_trait]
    impl IDFTTxStorageAPI for DFTTxStorageAPI {
        async fn batch_append(&self, storage_canister_id: Principal, blocks: VecDeque<EncodedBlock>) -> CommonResult<()>;
    }
}

#[fixture]
pub fn mock_dft_tx_storage_api() -> MockDFTTxStorageAPI {
    MockDFTTxStorageAPI::new()
}

#[fixture]
pub fn service(test_token_id: Principal) -> AutoScalingStorageService {
    init_test();
    AutoScalingStorageService::new(test_token_id)
}

#[rstest]
async fn test_auto_scaling_storage_with_create_storage_fail(
    mut service: AutoScalingStorageService,
    mut mock_dft_tx_storage_api: MockDFTTxStorageAPI,
    mut mock_ic_management_api: MockICManagementAPI,
    test_owner: Principal,
    other_caller: Principal,
    test_fee_to: Principal,
    now: u64,
) {
    test_token();

    mock_ic_management_api
        .expect_create_canister()
        .returning(|_| Err("create canister failed".to_string()));

    mock_dft_tx_storage_api
        .expect_batch_append()
        .returning(|_, _| Ok(()));

    service.ic_management = Arc::new(mock_ic_management_api);
    service.dft_tx_storage = Arc::new(mock_dft_tx_storage_api);
    for i in 0..=3000u64 {
        let new_fee_to = if i % 2u64 == 0u64 {
            TokenHolder::new(test_fee_to.clone(), None)
        } else {
            TokenHolder::new(other_caller.clone(), None)
        };
        let call_res =
            management_service::set_fee_to(&test_owner, new_fee_to, None, now.clone() + i);
        assert_eq!(call_res.is_ok(), true);
        service.exec_auto_scaling_strategy().await
    }

    assert_eq!(
        blockchain_service::archived_blocks_num(),
        BigUint::from(0u32)
    );
}

#[rstest]
async fn test_auto_scaling_storage_with_create_storage_success_and_install_fail(
    mut service: AutoScalingStorageService,
    mut mock_dft_tx_storage_api: MockDFTTxStorageAPI,
    mut mock_ic_management_api: MockICManagementAPI,
    test_owner: Principal,
    other_caller: Principal,
    test_fee_to: Principal,
    now: u64,
) {
    test_token();

    mock_ic_management_api
        .expect_create_canister()
        .returning(|_| {
            Ok(CanisterIdRecord {
                canister_id: Principal::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap(),
            })
        });

    mock_ic_management_api
        .expect_canister_install()
        .returning(|_, _, _| Err("install canister failed".to_string()));

    mock_dft_tx_storage_api
        .expect_batch_append()
        .returning(|_, _| Ok(()));

    service.ic_management = Arc::new(mock_ic_management_api);
    service.dft_tx_storage = Arc::new(mock_dft_tx_storage_api);
    for i in 0..=3000u64 {
        let new_fee_to = if i % 2u64 == 0u64 {
            TokenHolder::new(test_fee_to.clone(), None)
        } else {
            TokenHolder::new(other_caller.clone(), None)
        };
        let call_res =
            management_service::set_fee_to(&test_owner, new_fee_to, None, now.clone() + i);
        assert_eq!(call_res.is_ok(), true);
        service.exec_auto_scaling_strategy().await
    }

    assert_eq!(
        blockchain_service::archived_blocks_num(),
        BigUint::from(0u32)
    );
}

#[rstest]
async fn test_auto_scaling_storage_with_create_storage_success_and_install_success_and_batch_append_failed(
    mut service: AutoScalingStorageService,
    mut mock_dft_tx_storage_api: MockDFTTxStorageAPI,
    mut mock_ic_management_api: MockICManagementAPI,
    test_owner: Principal,
    other_caller: Principal,
    test_fee_to: Principal,
    now: u64,
) {
    test_token();

    mock_ic_management_api
        .expect_create_canister()
        .times(1)
        .returning(|_| {
            Ok(CanisterIdRecord {
                canister_id: test_auto_scaling_storage_id(),
            })
        });

    mock_ic_management_api
        .expect_canister_install()
        .times(1)
        .returning(|_, _, _| Ok(()));

    mock_ic_management_api
        .expect_canister_status()
        .returning(|_| {
            Ok(CanisterStatusResponse {
                status: CanisterStatus::Running,
                settings: CanisterSettings {
                    controllers: None,
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                },
                module_hash: None,
                controller: test_token_id(),
                memory_size: MIN_CANISTER_STORAGE_BYTES.into(),
                cycles: 0u32.into(),
            })
        });

    mock_dft_tx_storage_api
        .expect_batch_append()
        .returning(|_, _| Err(DFTError::MoveTxToScalingStorageFailed));

    service.ic_management = Arc::new(mock_ic_management_api);
    service.dft_tx_storage = Arc::new(mock_dft_tx_storage_api);
    for i in 0..=3000u64 {
        let new_fee_to = if i % 2u64 == 0u64 {
            TokenHolder::new(test_fee_to.clone(), None)
        } else {
            TokenHolder::new(other_caller.clone(), None)
        };
        let call_res =
            management_service::set_fee_to(&test_owner, new_fee_to, None, now.clone() + i);
        assert_eq!(call_res.is_ok(), true);
        if i >= 2000u64 {
            assert_eq!(
                blockchain_service::last_auto_scaling_storage_canister_id().unwrap(),
                test_auto_scaling_storage_id()
            );
        }
        service.exec_auto_scaling_strategy().await;
    }

    assert_eq!(
        blockchain_service::archived_blocks_num(),
        BigUint::from(0u32)
    );
}

#[rstest]
async fn test_auto_scaling_storage_with_create_storage_success_and_install_success_and_batch_append_success(
    mut service: AutoScalingStorageService,
    mut mock_dft_tx_storage_api: MockDFTTxStorageAPI,
    mut mock_ic_management_api: MockICManagementAPI,
    test_owner: Principal,
    other_caller: Principal,
    test_fee_to: Principal,
    now: u64,
) {
    test_token();

    mock_ic_management_api
        .expect_create_canister()
        .times(1)
        .returning(|_| {
            Ok(CanisterIdRecord {
                canister_id: test_auto_scaling_storage_id(),
            })
        });

    mock_ic_management_api
        .expect_canister_install()
        .times(1)
        .returning(|_, _, _| Ok(()));

    mock_ic_management_api
        .expect_canister_status()
        .returning(|_| {
            Ok(CanisterStatusResponse {
                status: CanisterStatus::Running,
                settings: CanisterSettings {
                    controllers: None,
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                },
                module_hash: None,
                controller: test_token_id(),
                memory_size: MIN_CANISTER_STORAGE_BYTES.into(),
                cycles: 0u32.into(),
            })
        });

    mock_dft_tx_storage_api
        .expect_batch_append()
        .returning(|_, _| Ok(()));

    service.ic_management = Arc::new(mock_ic_management_api);
    service.dft_tx_storage = Arc::new(mock_dft_tx_storage_api);
    for i in 0..=3000u64 {
        let new_fee_to = if i % 2u64 == 0u64 {
            TokenHolder::new(test_fee_to.clone(), None)
        } else {
            TokenHolder::new(other_caller.clone(), None)
        };
        let call_res =
            management_service::set_fee_to(&test_owner, new_fee_to, None, now.clone() + i);
        assert_eq!(call_res.is_ok(), true);
        if i >= 2000u64 {
            assert_eq!(
                blockchain_service::last_auto_scaling_storage_canister_id().unwrap(),
                test_auto_scaling_storage_id()
            );
        }
        service.exec_auto_scaling_strategy().await;
    }

    assert_eq!(
        blockchain_service::archived_blocks_num(),
        BigUint::from(2000u32)
    );
}

#[rstest]
async fn test_auto_scaling_storage_with_create_storage_success_and_install_success_increase_twice(
    mut service: AutoScalingStorageService,
    mut mock_dft_tx_storage_api: MockDFTTxStorageAPI,
    mut mock_ic_management_api: MockICManagementAPI,
    test_owner: Principal,
    other_caller: Principal,
    test_fee_to: Principal,
    now: u64,
) {
    test_token();
<<<<<<< HEAD
    let mut toggle_return = false;

    mock_ic_management_api
        .expect_create_canister()
        .times(2)
        .returning(move |_| {
            if !toggle_return {
                toggle_return = true;
                Ok(CanisterIdRecord {
                    canister_id: test_auto_scaling_storage_id(),
                })
            } else {
                Ok(CanisterIdRecord {
                    canister_id: test_auto_scaling_storage_id2(),
                })
            }
        });
=======

    mock_ic_management_api
        .expect_create_canister()

        .return_once(|_| {
            Ok(CanisterIdRecord {
                canister_id: test_auto_scaling_storage_id2(),
            })
        })
        .return_once(|_| {
            Ok(CanisterIdRecord {
                canister_id: test_auto_scaling_storage_id(),
            })
        });

>>>>>>> 202560d (Unit Test: auto scaling storage)
    mock_ic_management_api
        .expect_canister_install()
        .returning(move |_, _, _| Ok(()));

    mock_ic_management_api
        .expect_canister_status()
        .return_once(move |_| {
            Ok(CanisterStatusResponse {
                status: CanisterStatus::Running,
                settings: CanisterSettings {
                    controllers: None,
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                },
                module_hash: None,
                controller: test_token_id(),
<<<<<<< HEAD
                memory_size: (MAX_CANISTER_STORAGE_BYTES - 161000u32)
                    .into(),
                cycles: 0u32.into(),
            })
        });
    mock_ic_management_api
        .expect_canister_status()
=======
                memory_size: (MAX_CANISTER_STORAGE_BYTES - MIN_CANISTER_STORAGE_BYTES - 100).into(),
                cycles: 0u32.into(),
            })
        })
>>>>>>> 202560d (Unit Test: auto scaling storage)
        .return_once(move |_| {
            Ok(CanisterStatusResponse {
                status: CanisterStatus::Running,
                settings: CanisterSettings {
                    controllers: None,
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                },
                module_hash: None,
                controller: test_token_id(),
<<<<<<< HEAD
                memory_size: (MAX_CANISTER_STORAGE_BYTES - 100).into(),
                cycles: 0u32.into(),
            })
        });
=======
                memory_size: (MAX_CANISTER_STORAGE_BYTES - MIN_CANISTER_STORAGE_BYTES - 161000u32)
                    .into(),
                cycles: 0u32.into(),
            })
        });

>>>>>>> 202560d (Unit Test: auto scaling storage)
    mock_dft_tx_storage_api
        .expect_batch_append()
        .returning(move |_, _| Ok(()));

    service.ic_management = Arc::new(mock_ic_management_api);
    service.dft_tx_storage = Arc::new(mock_dft_tx_storage_api);
    for i in 0..=3000u64 {
        let new_fee_to = if i % 2u64 == 0u64 {
            TokenHolder::new(test_fee_to.clone(), None)
        } else {
            TokenHolder::new(other_caller.clone(), None)
        };
        let call_res =
            management_service::set_fee_to(&test_owner, new_fee_to, None, now.clone() + i);
        assert_eq!(call_res.is_ok(), true);
        service.exec_auto_scaling_strategy().await;

<<<<<<< HEAD
        if i >= 2000u64 && i < 2999u64 {
            assert_eq!(
                blockchain_service::last_auto_scaling_storage_canister_id().unwrap(),
                test_auto_scaling_storage_id(),
                "last {},test {}", blockchain_service::last_auto_scaling_storage_canister_id().unwrap().to_text(), test_auto_scaling_storage_id().to_text()
=======
        if i >= 2000u64 {
            assert_eq!(
                blockchain_service::last_auto_scaling_storage_canister_id().unwrap(),
                test_auto_scaling_storage_id()
>>>>>>> 202560d (Unit Test: auto scaling storage)
            );
        }
        if i >= 3000u64 {
            assert_eq!(
                blockchain_service::last_auto_scaling_storage_canister_id().unwrap(),
                test_auto_scaling_storage_id2()
            );
        }
    }

    assert_eq!(
        blockchain_service::archived_blocks_num(),
        BigUint::from(2000u32)
    );
}
