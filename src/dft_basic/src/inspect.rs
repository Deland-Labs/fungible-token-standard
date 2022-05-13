use crate::service::basic_service;
use candid::{Nat, Principal};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::inspect_message;
use log::{error, info};

static QUERY_METHODS: [&str; 19] = [
    "allowance",
    "allowancesOf",
    "archives",
    "balanceOf",
    "decimals",
    "desc",
    "fee",
    "logo",
    "meta",
    "minters",
    "name",
    "owner",
    "symbol",
    "tokenInfo",
    "totalSupply",
    "blockByHeight",
    "blocksByQuery",
    "http_request",
    "__get_candid_interface_tmp_hack",
];

static OWNER_METHODS: [&str; 6] = [
    "setDesc", "setFee", "setFeeTo", "setFeeTo", "setLogo", "setOwner",
];
static HOLDER_METHODS: [&str; 3] = ["approve", "transfer", "burn"];

//static SPENDER_METHODS: [&str; 3] = ["transferFrom", "burnFrom"];
#[inspect_message]
fn inspect_message() {
    let method = api::call::method_name();
    let caller = api::caller();

    match inspect_message_inner(&method, caller) {
        Ok(()) => api::call::accept_message(),
        Err(e) => api::call::reject(&e),
    }
}

fn inspect_message_inner(method: &String, caller: Principal) -> Result<(), String> {
    match &method[..] {
        m if QUERY_METHODS.contains(&m) => Ok(()),
        m if HOLDER_METHODS.contains(&m) => {
            let holder = match m {
                "approve" => {
                    let (sub_account, _, _) =
                        api::call::arg_data::<(Option<Subaccount>, String, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                "transfer" => {
                    let (sub_account, _, _) =
                        api::call::arg_data::<(Option<Subaccount>, String, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                "burn" => {
                    let (sub_account, _) = api::call::arg_data::<(Option<Subaccount>, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                // never match this case
                _ => TokenHolder::new(caller, None),
            };

            // check caller's balance
            let balance = basic_service::balance_of(&holder);
            if balance > TokenAmount::from(0u32) {
                Ok(())
            } else if caller == Principal::anonymous() {
                let err: ErrorInfo = DFTError::NotAllowAnonymous.into();
                let err_msg = format!("reject {:?}", err);
                error!("method {} {}", method, err_msg);
                Err(err_msg)
            } else {
                let err: ErrorInfo = DFTError::InsufficientBalance.into();
                let err_msg = format!("reject {:?}", err);
                error!("method {} {}", method, err_msg);
                Err(err_msg)
            }
        }
        m if OWNER_METHODS.contains(&m) => {
            // check if caller is owner
            let owner = basic_service::owner();
            if caller == owner {
                Ok(())
            } else {
                let err: ErrorInfo = DFTError::OnlyOwnerAllowCallIt.into();
                let err_msg = format!("reject {:?}", err);
                error!("method {} {}", method, err_msg);
                Err(err_msg)
            }
        }
        _ => {
            info!("inspect: method {} not checked; accept", method);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect_message_with_caller() {
        let caller: Principal = "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
            .parse()
            .unwrap();

        for method in QUERY_METHODS.iter() {
            let method_name = method.to_string();
            let result = inspect_message_inner(&method_name, caller.clone());
            assert!(result.is_ok());
        }

        for method in OWNER_METHODS.iter() {
            let method_name = method.to_string();
            let result = inspect_message_inner(&method_name, caller.clone());
            assert!(result.is_err());
        }

        let method_name = "unknown_method".to_string();
        let result = inspect_message_inner(&method_name, caller.clone());
        assert!(result.is_ok());
    }
}
