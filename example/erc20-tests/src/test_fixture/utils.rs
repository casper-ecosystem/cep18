use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_ADDR, DEFAULT_PAYMENT,
};
use casper_execution_engine::{
    core::engine_state::{DeployItem, ExecuteRequest},
    storage::global_state::in_memory::InMemoryGlobalState,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::FromBytes,
    runtime_args,
    system::{handle_payment::ARG_TARGET, mint::ARG_ID},
    AsymmetricType, CLTyped, Key, PublicKey, RuntimeArgs, U512,
};

pub fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    contract_key: &Key,
    dictionary_name: &str,
    dictionary_key: &str,
) -> Option<T> {
    let seed_uref = *builder
        .query(None, *contract_key, &[])
        .expect("must have contract")
        .as_contract()
        .expect("must convert as contract")
        .named_keys()
        .get(dictionary_name)
        .expect("must have key")
        .as_uref()
        .expect("must convert to seed uref");
    let value = builder.query_dictionary_item(None, seed_uref, dictionary_key);
    if let Ok(value) = value {
        let value = value
            .as_cl_value()
            .expect("should be CLValue")
            .to_owned()
            .into_t::<T>()
            .unwrap();
        Some(value)
    } else {
        None
    }
}

pub fn fund_account(account: &AccountHash) -> ExecuteRequest {
    let deploy_item = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_address(*DEFAULT_ACCOUNT_ADDR)
        .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
        .with_transfer_args(runtime_args! {
            ARG_AMOUNT => U512::from(30_000_000_000_000_u64),
            ARG_TARGET => *account,
            ARG_ID => <Option::<u64>>::None
        })
        .with_deploy_hash([1; 32])
        .build();
    ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
}

pub fn get_accounts(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
) -> (AccountHash, AccountHash, AccountHash) {
    let ali = PublicKey::ed25519_from_bytes([3u8; 32])
        .unwrap()
        .to_account_hash();
    let bob = PublicKey::ed25519_from_bytes([6u8; 32])
        .unwrap()
        .to_account_hash();
    let joe = PublicKey::ed25519_from_bytes([9u8; 32])
        .unwrap()
        .to_account_hash();

    builder.exec(fund_account(&ali)).expect_success().commit();
    builder.exec(fund_account(&bob)).expect_success().commit();
    builder.exec(fund_account(&joe)).expect_success().commit();
    (ali, bob, joe)
}

pub fn execute_request(builder: &mut InMemoryWasmTestBuilder, deploy_item: DeployItem) {
    let execute_request_builder = ExecuteRequestBuilder::from_deploy_item(deploy_item);
    let exec = builder.exec(execute_request_builder.build());
    exec.expect_success().commit();
}
