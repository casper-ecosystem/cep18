use crate::utility::{
    constants::{
        CEP18_CONTRACT_WASM, CEP18_TEST_TOKEN_CONTRACT_VERSION, TOKEN_DECIMALS, TOKEN_NAME,
        TOKEN_SYMBOL, TOKEN_TOTAL_SUPPLY,
    },
    installer_request_builders::{setup, setup_with_args},
    support::query_stored_value,
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, Key, U256};
use cep18::{
    constants::{
        ARG_DECIMALS, ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_SYMBOL, ARG_TOTAL_SUPPLY,
    },
    modalities::EventsMode,
};

#[test]
fn should_upgrade_contract_version() {
    let (mut builder, ..) = setup();

    let version_0_string: String = query_stored_value(
        &builder,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
        CEP18_TEST_TOKEN_CONTRACT_VERSION,
    );

    let parts: Vec<&str> = version_0_string.split('.').collect();

    let version_0_major: u32 = parts
        .first()
        .expect("Failed to get the major version")
        .parse()
        .expect("Failed to parse the major version as u32");

    let version_0_minor: u32 = parts
        .get(1)
        .unwrap_or(&"0") // Default to "0" if no minor version exists
        .parse()
        .expect("Failed to parse the minor version as u32");

    let upgrade_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_CONTRACT_WASM,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
        },
    )
    .build();

    builder.exec(upgrade_request).expect_success().commit();

    let version_1_string: String = query_stored_value(
        &builder,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
        CEP18_TEST_TOKEN_CONTRACT_VERSION,
    );

    // Split into major and minor parts
    let parts: Vec<&str> = version_1_string.split('.').collect();

    // Parse the major and minor components
    let version_1_major: u32 = parts
        .first()
        .expect("Failed to get the major version")
        .parse()
        .expect("Failed to parse the major version as u32");

    let version_1_minor: u32 = parts
        .get(1)
        .unwrap_or(&"0") // Default to "0" if no minor version exists
        .parse()
        .expect("Failed to parse the minor version as u32");

    assert!(version_0_major == version_1_major);
    assert!(version_0_minor < version_1_minor);
}

#[test]
fn should_upgrade_contract_from_ces_to_native() {
    let (mut builder, ..) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ARG_EVENTS_MODE => EventsMode::CES as u8,
        ARG_ENABLE_MINT_BURN => true,
    });

    let upgrade_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_CONTRACT_WASM,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_EVENTS_MODE => EventsMode::Native as u8,
        },
    )
    .build();

    builder.exec(upgrade_request).expect_success().commit();
}
