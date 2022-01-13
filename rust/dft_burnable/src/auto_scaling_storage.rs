extern crate dft_types;
extern crate dft_utils;

use candid::encode_args;
use dft_standard::{ic_management::*};
use dft_types::constants::{CYCLES_PER_AUTO_SCALING, MAX_HEAP_MEMORY_SIZE, MAX_TXS_CACHE_IN_DFT};
use dft_types::*;
use ic_cdk::{
    api,
    export::{candid::Nat, Principal},
};
use crate::state::TOKEN;

pub(crate) async fn exec_auto_scaling_strategy() -> ActorResult<()> {
    let inner_txs: Vec<TxRecord> = TOKEN.with(|token| {
        let token = token.borrow();
        token.get_inner_txs()
    });
    let first_tx_index_inner: Nat = TOKEN.with(|token| {
        let token = token.borrow();
        token.get_tx_index(&inner_txs[0])
    });

    // When create auto-scaling storage ?
    // DFT's txs count > 2000
    // It's means when creating a test DFT, when the number of transactions is less than 2000, no storage will be created to save cycles
    if inner_txs.len() >= MAX_TXS_CACHE_IN_DFT * 2 {
        let storage_canister_id = get_or_create_available_storage_id(&first_tx_index_inner).await?;

        let should_save_txs = inner_txs[0..MAX_TXS_CACHE_IN_DFT].to_vec();
        //save the txs to auto-scaling storage
        match api::call::call(storage_canister_id, "batchAppend", (should_save_txs,)).await {
            Ok((res,)) => {
                if res {
                    TOKEN.with(|token| {
                        let mut token = token.borrow_mut();
                        (0..MAX_TXS_CACHE_IN_DFT).for_each(|_| token.remove_inner_txs(0));
                    });
                }
            }
            Err((_, emsg)) => {
                api::print(format!(
                    "batchAppend: save to auto-scaling storage failed,{}  ",
                    emsg
                ));
            }
        }
    }

    Ok(())
}

async fn get_or_create_available_storage_id(tx_index: &Nat) -> ActorResult<Principal> {
    let mut max_key = Nat::from(0);
    let mut last_storage_id = Principal::anonymous();
    TOKEN.with(|token| {
        let token = token.borrow();
        for (k, v) in token.get_storage_canister_ids() {
            if k >= max_key && last_storage_id != v {
                max_key = k.clone();
                last_storage_id = v.clone();
            }
        }
    });
    let mut is_necessary_create_new_storage_canister = last_storage_id == Principal::anonymous();

    // check storage remain size
    if !is_necessary_create_new_storage_canister {
        let req = CanisterIdRecord {
            canister_id: last_storage_id,
        };
        let status = get_canister_status(req).await;
        match status {
            Ok(res) => {
                ic_cdk::print(format!("memory_size is {}", res.memory_size));
                let min_storage_size_for_cache_txs =
                    Nat::from(MAX_TXS_CACHE_IN_DFT * std::mem::size_of::<TxRecord>());

                if (Nat::from(MAX_HEAP_MEMORY_SIZE) - res.memory_size)
                    .lt(&min_storage_size_for_cache_txs)
                {
                    is_necessary_create_new_storage_canister = true;
                } else {
                    return Ok(last_storage_id);
                }
            }
            Err(_) => {
                return Err(DFTError::StorageScalingFailed.into());
            }
        };
    }

    if is_necessary_create_new_storage_canister {
        const STORAGE_WASM: &[u8] = std::include_bytes!(
            "../../target/wasm32-unknown-unknown/release/dft_tx_storage_opt.wasm"
        );
        let dft_id = api::id();
        let create_args = CreateCanisterArgs {
            cycles: CYCLES_PER_AUTO_SCALING,
            settings: CanisterSettings {
                controllers: Some(vec![dft_id.clone()]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            },
        };
        api::print("creating token storage...");
        let create_result = create_canister(create_args).await;

        match create_result {
            Ok(cdr) => {
                api::print(format!(
                    "token new storage canister id : {} ,start index is {}",
                    cdr.canister_id.clone().to_string(),
                    tx_index.clone()
                ));

                let install_args = encode_args((dft_id.clone(), tx_index.clone()))
                    .expect("Failed to encode arguments.");

                match install_canister(&cdr.canister_id, STORAGE_WASM.to_vec(), install_args).await
                {
                    Ok(_) => {
                        TOKEN.with(|token| {
                            let mut token = token.borrow_mut();
                            token.add_storage_canister_ids(tx_index.clone(), cdr.canister_id)
                        });
                        return Ok(cdr.canister_id);
                    }
                    Err(emsg) => {
                        api::print(format!(
                            "install auto-scaling storage canister failed. details:{}",
                            emsg
                        ));
                        return Err(DFTError::StorageScalingFailed.into());
                    }
                }
            }
            Err(emsg) => {
                api::print(format!("create new storage canister failed {}", emsg).as_str());
                return Err(DFTError::StorageScalingFailed.into());
            }
        };
    } else {
        return Ok(last_storage_id);
    }
}
