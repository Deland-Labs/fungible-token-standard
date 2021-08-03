#[warn(unused_must_use)]
use sudograph::graphql_database;

graphql_database!("canisters/graphql/src/schema.graphql");

static mut OWNER: sudograph::ic_cdk::export::Principal =
    sudograph::ic_cdk::export::Principal::anonymous();
static mut TOKEN_CANISTER_ID: sudograph::ic_cdk::export::Principal =
    sudograph::ic_cdk::export::Principal::anonymous();

#[sudograph::ic_cdk_macros::init]
async fn init_custom() {
    unsafe {
        OWNER = sudograph::ic_cdk::caller();
        init().await
    }
}

#[sudograph::ic_cdk_macros::update(name = "graphql_mutation")]
async fn graphql_mutation_custom(mutation_string: String, variables_json_string: String) -> String {
    unsafe {
        assert!(TOKEN_CANISTER_ID == sudograph::ic_cdk::caller());
    }
    return graphql_mutation(mutation_string, variables_json_string).await;
}

#[sudograph::ic_cdk_macros::update]
async fn set_token_canister_id(token: sudograph::ic_cdk::export::Principal) -> bool {
    unsafe {
        assert!(OWNER == sudograph::ic_cdk::caller());
        TOKEN_CANISTER_ID = token;
    }
    true
}

#[sudograph::ic_cdk_macros::pre_upgrade]
async fn pre_upgrade_custom() {
    unsafe {
        sudograph::ic_cdk::storage::stable_save((OWNER, TOKEN_CANISTER_ID)).unwrap();
    }
}

#[sudograph::ic_cdk_macros::post_upgrade]
async fn post_upgrade_costom() {
    let (owner, token_canister_id): (
        sudograph::ic_cdk::export::Principal,
        sudograph::ic_cdk::export::Principal,
    ) = sudograph::ic_cdk::storage::stable_restore().unwrap();
    unsafe {
        OWNER = owner;
        TOKEN_CANISTER_ID = token_canister_id;
    }
    post_upgrade().await
}
