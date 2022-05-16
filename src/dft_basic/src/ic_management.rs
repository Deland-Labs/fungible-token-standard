#![allow(dead_code)]
use ic_cdk::api;
use ic_cdk::export::candid::{CandidType, Nat, Principal};
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct CanisterIdRecord {
    pub canister_id: Principal,
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<Nat>,
    pub memory_allocation: Option<Nat>,
    pub freezing_threshold: Option<Nat>,
}

#[allow(non_camel_case_types)]
#[derive(CandidType, Debug, Deserialize)]
pub enum CanisterStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "stopped")]
    Stopped,
}

#[derive(CandidType, Debug, Deserialize)]
pub struct CanisterStatusResponse {
    pub status: CanisterStatus,
    pub settings: CanisterSettings,
    pub module_hash: Option<Vec<u8>>,
    pub controller: Principal,
    pub memory_size: Nat,
    pub cycles: Nat,
}

// Install Wasm
#[derive(CandidType, Deserialize)]
enum InstallMode {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
}

#[derive(CandidType, Deserialize)]
struct CanisterInstall {
    mode: InstallMode,
    canister_id: Principal,
    #[serde(with = "serde_bytes")]
    wasm_module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    arg: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CreateCanisterArgs {
    pub cycles: u64,
    pub settings: CanisterSettings,
}

pub async fn get_canister_status(
    id_record: CanisterIdRecord,
) -> Result<CanisterStatusResponse, String> {
    let res: Result<(CanisterStatusResponse,), _> = api::call::call(
        Principal::management_canister(),
        "canister_status",
        (id_record,),
    )
    .await;
    match res {
        Ok(x) => Ok(x.0),
        Err((code, msg)) => Err(format!(
            "An error happened during the call: {}: {}",
            code as u8, msg
        )),
    }
}

pub async fn create_canister(args: CreateCanisterArgs) -> Result<CanisterIdRecord, String> {
    #[derive(CandidType)]
    struct In {
        settings: Option<CanisterSettings>,
    }
    let in_arg = In {
        settings: Some(args.settings),
    };

    let (create_result,): (CanisterIdRecord,) = match api::call::call_with_payment(
        Principal::management_canister(),
        "create_canister",
        (in_arg,),
        args.cycles,
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };

    Ok(create_result)
}

pub async fn install_canister(
    canister_id: &Principal,
    wasm_module: Vec<u8>,
    args: Vec<u8>,
) -> Result<(), String> {
    let install_config = CanisterInstall {
        mode: InstallMode::Install,
        canister_id: *canister_id,
        wasm_module: wasm_module.clone(),
        arg: args,
    };

    match api::call::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };
    Ok(())
}
