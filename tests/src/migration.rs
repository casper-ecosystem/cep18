use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        ARG_DECIMALS, ARG_NAME, ARG_SYMBOL, ARG_TOTAL_SUPPLY, CEP18_CONTRACT_WASM, TOKEN_DECIMALS,
        TOKEN_NAME, TOKEN_SYMBOL, TOKEN_TOTAL_SUPPLY,
    },
    installer_request_builders::{setup, TestContext},
};

#[test]
fn should_upgrade_contract_version() {
    let (mut builder, TestContext { cep18_token: _, .. }) = setup();

    let version_0: u32 = builder
        .query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &["cep18_contract_version_CasperTest".to_string()],
        )
        .unwrap()
        .as_cl_value()
        .unwrap()
        .clone()
        .into_t()
        .unwrap();

    let upgrade_request = ExecuteRequestBuilder::standard(
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

    builder.exec(upgrade_request).expect_success().commit();

    let version_1: u32 = builder
        .query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &["cep18_contract_version_CasperTest".to_string()],
        )
        .unwrap()
        .as_cl_value()
        .unwrap()
        .clone()
        .into_t()
        .unwrap();

    assert!(version_0 < version_1);
}
