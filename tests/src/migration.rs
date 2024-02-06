use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::runtime_args;

use crate::utility::{
    constants::{
        ARG_DECIMALS, ARG_NAME, ARG_SYMBOL, ARG_TOTAL_SUPPLY, CEP18_CONTRACT_WASM,
        CEP18_TOKEN_CONTRACT_KEY, DECIMALS_KEY, NAME_KEY, SYMBOL_KEY, TOKEN_DECIMALS, TOKEN_NAME,
        TOKEN_SYMBOL, TOKEN_TOTAL_SUPPLY, TOTAL_SUPPLY_KEY,
    },
    installer_request_builders::setup,
};

#[test]
fn should_have_queryable_properties() {
    let (mut builder, TestContext { cep18_token, .. }) = setup();
    let pre_account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let version_0 = account
        .named_keys()
        .get("cep18_contract_version_CasperTest")
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let install_request_1 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_CONTRACT_WASM,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_SYMBOL => TOKEN_SYMBOL,
            ARG_DECIMALS => TOKEN_DECIMALS,
            ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        },
    )
    .build();

    let post_account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let version_1 = account
        .named_keys()
        .get("cep18_contract_version_CasperTest")
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    assert!(version_0 < version_1);
}
