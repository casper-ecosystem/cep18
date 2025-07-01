use casper_engine_test_support::{
    utils::create_run_genesis_request, ExecuteRequestBuilder, LmdbWasmTestBuilder,
    DEFAULT_ACCOUNTS, DEFAULT_ACCOUNT_ADDR,
};
use casper_execution_engine::{engine_state::Error as CoreError, execution::ExecError};
use casper_types::{runtime_args, system::mint::TOTAL_SUPPLY_KEY, ApiError, EntityAddr, Key, U256};
use cep18::{
    constants::{
        ARG_DECIMALS, ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_SYMBOL,
        ARG_TOTAL_SUPPLY, DICT_ALLOWANCES, DICT_BALANCES,
    },
    modalities::EventsMode,
};

use crate::utility::{
    constants::{
        CEP18_CONTRACT_WASM, TOKEN_DECIMALS, TOKEN_NAME, TOKEN_SYMBOL, TOKEN_TOTAL_SUPPLY,
    },
    installer_request_builders::{
        cep18_check_balance_of, invert_cep18_address, setup, TestContext,
    },
};

#[test]
fn should_have_queryable_properties() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup();

    let cep18_entity_addr = EntityAddr::new_smart_contract(cep18_contract_hash.value());

    let name: String = builder.get_value(cep18_entity_addr, ARG_NAME);
    assert_eq!(name, TOKEN_NAME);

    let symbol: String = builder.get_value(cep18_entity_addr, ARG_SYMBOL);
    assert_eq!(symbol, TOKEN_SYMBOL);

    let decimals: u8 = builder.get_value(cep18_entity_addr, ARG_DECIMALS);
    assert_eq!(decimals, TOKEN_DECIMALS);

    let total_supply: U256 = builder.get_value(cep18_entity_addr, TOTAL_SUPPLY_KEY);
    assert_eq!(total_supply, U256::from(TOKEN_TOTAL_SUPPLY));

    let owner_key = Key::Account(*DEFAULT_ACCOUNT_ADDR);

    let owner_balance = cep18_check_balance_of(&mut builder, &cep18_contract_hash, owner_key);
    assert_eq!(owner_balance, total_supply);

    let contract_balance = cep18_check_balance_of(
        &mut builder,
        &cep18_contract_hash,
        Key::Hash(cep18_contract_hash.value()),
    );
    assert_eq!(contract_balance, U256::zero());

    // Ensures that Account and Contract ownership is respected and we're not keying ownership under
    // the raw bytes regardless of variant.
    let inverted_owner_key = invert_cep18_address(owner_key);
    let inverted_owner_balance =
        cep18_check_balance_of(&mut builder, &cep18_contract_hash, inverted_owner_key);
    assert_eq!(inverted_owner_balance, U256::zero());
}

#[test]
fn should_not_store_balances_or_allowances_under_account_after_install() {
    let (builder, _contract_hash) = setup();

    let named_keys = builder.get_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR);

    assert!(!named_keys.contains(DICT_BALANCES), "{named_keys:?}",);
    assert!(!named_keys.contains(DICT_ALLOWANCES), "{named_keys:?}");
}

#[test]
fn should_fail_with_left_over_bytes_converted_into_60006() {
    let mut builder = LmdbWasmTestBuilder::default();
    builder
        .run_genesis(create_run_genesis_request(DEFAULT_ACCOUNTS.to_vec()))
        .commit();

    let install_request_1 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_CONTRACT_WASM,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_SYMBOL => TOKEN_SYMBOL,
            ARG_DECIMALS => TOKEN_DECIMALS,
            ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
            ARG_EVENTS_MODE => Some(EventsMode::Native as u8),
            ARG_ENABLE_MINT_BURN => true,
        },
    )
    .build();

    builder.exec(install_request_1).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(
            error,
            CoreError::Exec(ExecError::Revert(ApiError::User(60006)))
        ),
        "{error:?}",
    );
}
