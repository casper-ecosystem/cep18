use std::collections::BTreeMap;

use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR, MINIMUM_ACCOUNT_CREATION_BALANCE, PRODUCTION_RUN_GENESIS_REQUEST, InMemoryWasmTestBuilder};
use casper_types::{U256, runtime_args, RuntimeArgs, Key, system::mint, ContractHash, ContractPackageHash};

use crate::utility::{installer_request_builders::{cep18_check_balance_of, cep18_check_total_supply, get_dictionary_value_from_key, cep18_check_allowance_of, make_cep18_approve_request}, constants::{METHOD_MINT, TOKEN_OWNER_ADDRESS_1, OWNER, AMOUNT, TOKEN_OWNER_AMOUNT_1, TOKEN_OWNER_AMOUNT_2, TOKEN_OWNER_ADDRESS_2, TOKEN_TOTAL_SUPPLY, ARG_OWNER, ARG_AMOUNT, METHOD_BURN, ALLOWANCE_AMOUNT_1, ALLOWANCE_AMOUNT_2, DECREASE_ALLOWANCE, ARG_SPENDER, INCREASE_ALLOWANCE, ARG_TOTAL_SUPPLY, ARG_DECIMALS, ARG_NAME, ARG_SYMBOL, TOKEN_NAME, TOKEN_SYMBOL, TOKEN_DECIMALS, CEP18_TEST_CONTARCT_WASM, ACCOUNT_1_ADDR, ACCOUNT_2_ADDR, CEP18_CONTRACT_WASM, CEP18_TOKEN_CONTRACT_KEY, CEP18_TOKEN_CONTRACT_PACKAGE_KEY, EVENTS_MODE}};

#[test]
fn test_events_cep47() {
    let mint_amount = U256::one();

    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);

    let id: Option<u64> = None;
    let transfer_1_args = runtime_args! {
        mint::ARG_TARGET => *ACCOUNT_1_ADDR,
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };
    let transfer_2_args = runtime_args! {
        mint::ARG_TARGET => *ACCOUNT_2_ADDR,
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };

    let transfer_request_1 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_1_args).build();
    let transfer_request_2 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_2_args).build();

    let install_request_1 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_CONTRACT_WASM,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_SYMBOL => TOKEN_SYMBOL,
            ARG_DECIMALS => TOKEN_DECIMALS,
            ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
            EVENTS_MODE => 1_u8
        },
    )
    .build();

    let install_request_2 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_TEST_CONTARCT_WASM,
        RuntimeArgs::default(),
    )
    .build();

    builder.exec(transfer_request_1).expect_success().commit();
    builder.exec(transfer_request_2).expect_success().commit();
    builder.exec(install_request_1).expect_success().commit();
    builder.exec(install_request_2).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep18_token = account
        .named_keys()
        .get(CEP18_TOKEN_CONTRACT_KEY)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let cep18_token_package = account
        .named_keys()
        .get(CEP18_TOKEN_CONTRACT_PACKAGE_KEY)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {OWNER => TOKEN_OWNER_ADDRESS_1, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();
    let mint_request_2 = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {OWNER => TOKEN_OWNER_ADDRESS_2, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_2)},
    )
    .build();
    builder.exec(mint_request_2).expect_success().commit();
    assert_eq!(
        cep18_check_balance_of(
            &mut builder,
            &cep18_token,
            Key::Account(*DEFAULT_ACCOUNT_ADDR)
        ),
        U256::from(TOKEN_TOTAL_SUPPLY),
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );
    let total_supply_before_mint = cep18_check_total_supply(&mut builder, &cep18_token);

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1) + mint_amount,
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );

    let total_supply_after_mint = cep18_check_total_supply(&mut builder, &cep18_token);
    assert_eq!(
        total_supply_after_mint,
        total_supply_before_mint + mint_amount,
    );
    let total_supply_before_burn = total_supply_after_mint;

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_BURN,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1),
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );
    let total_supply_after_burn = cep18_check_total_supply(&mut builder, &cep18_token);
    assert_eq!(
        total_supply_after_burn,
        total_supply_before_burn - mint_amount,
    );

    assert_eq!(total_supply_after_burn, total_supply_before_mint);
    let owner = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let spender = Key::Hash([42; 32]);
    let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
    let allowance_amount_2 = U256::from(ALLOWANCE_AMOUNT_2);

    let spender_allowance_before = cep18_check_allowance_of(&mut builder, owner, spender);
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request =
        make_cep18_approve_request(sender, &cep18_token, spender, allowance_amount_1);
    let decrease_allowance_request = ExecuteRequestBuilder::contract_call_by_hash(
        sender.into_account().unwrap(),
        cep18_token,
        DECREASE_ALLOWANCE,
        runtime_args! {
            ARG_SPENDER => spender,
            ARG_AMOUNT => allowance_amount_2,
        },
    )
    .build();
    let increase_allowance_request = ExecuteRequestBuilder::contract_call_by_hash(
        sender.into_account().unwrap(),
        cep18_token,
        INCREASE_ALLOWANCE,
        runtime_args! {
            ARG_SPENDER => spender,
            ARG_AMOUNT => allowance_amount_1,
        },
    )
    .build();

    builder.exec(approve_request).expect_success().commit();
    builder.exec(decrease_allowance_request).expect_success().commit();
    builder.exec(increase_allowance_request).expect_success().commit();

    // test mint event
    let mint_event = get_dictionary_value_from_key::<BTreeMap<String, String>>(
        &builder,
        &cep18_token.into(),
        "events",
        "2",
    );

    let mut expected_mint_event: BTreeMap<String, String> = BTreeMap::new();

    expected_mint_event.insert("event_type".to_string(), "mint".to_string());
    expected_mint_event.insert("cep18_package".to_string(), cep18_token_package.to_string());
    expected_mint_event.insert(
        "recipient".to_string(),
        "Key::Account(2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a)"
            .to_string(),
    );

    expected_mint_event.insert("token_amount".to_string(), "1".to_string());

    assert_eq!(mint_event, expected_mint_event);

    // test burn event
    let burn_event = get_dictionary_value_from_key::<BTreeMap<String, String>>(
        &builder,
        &cep18_token.into(),
        "events",
        "3",
    );

    let mut expected_burn_event: BTreeMap<String, String> = BTreeMap::new();

    expected_burn_event.insert("event_type".to_string(), "burn".to_string());
    expected_burn_event.insert("cep18_package".to_string(), cep18_token_package.to_string());
    expected_burn_event.insert(
        "owner".to_string(),
        "Key::Account(2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a)"
            .to_string(),
    );

    expected_burn_event.insert("token_amount".to_string(), "1".to_string());
    assert_eq!(burn_event, expected_burn_event);

    // approve event

    let approve_event = get_dictionary_value_from_key::<BTreeMap<String, String>>(
        &builder,
        &cep18_token.into(),
        "events",
        "4",
    );

    let mut expected_approve_event: BTreeMap<String, String> = BTreeMap::new();

    expected_approve_event.insert("event_type".to_string(), "SetAllowance".to_string());
    expected_approve_event.insert("cep18_package".to_string(), cep18_token_package.to_string());
    expected_approve_event.insert(
        "owner".to_string(),
        "Key::Account(58b891759929bd4ed5a9cce20b9d6e3c96a66c21386bed96040e17dd07b79fa7)"
            .to_string(),
    );
    expected_approve_event.insert(
        "spender".to_string(),
        "Key::Hash(2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a)".to_string(),
    );
    expected_approve_event.insert("token_amount".to_string(), allowance_amount_1.to_string());

    assert_eq!(approve_event, expected_approve_event);

    // decrease allowance event
    let decrease_allowance_event = get_dictionary_value_from_key::<BTreeMap<String, String>>(
        &builder,
        &cep18_token.into(),
        "events",
        "5",
    );

    let mut expected_decrease_allowance_event: BTreeMap<String, String> = BTreeMap::new();

    expected_decrease_allowance_event
        .insert("event_type".to_string(), "DecreaseAllowance".to_string());
    expected_decrease_allowance_event
        .insert("cep18_package".to_string(), cep18_token_package.to_string());
    expected_decrease_allowance_event.insert(
        "owner".to_string(),
        "Key::Account(58b891759929bd4ed5a9cce20b9d6e3c96a66c21386bed96040e17dd07b79fa7)"
            .to_string(),
    );
    expected_decrease_allowance_event.insert(
        "spender".to_string(),
        "Key::Hash(2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a)".to_string(),
    );
    expected_decrease_allowance_event.insert(
        "token_amount".to_string(),
        (allowance_amount_1 - allowance_amount_2).to_string(),
    );
    expected_decrease_allowance_event.insert("decr_by".to_string(), allowance_amount_2.to_string());

    assert_eq!(decrease_allowance_event, expected_decrease_allowance_event);

    // increase allowance event

    let increase_allowance_event = get_dictionary_value_from_key::<BTreeMap<String, String>>(
        &builder,
        &cep18_token.into(),
        "events",
        "6",
    );

    let mut expected_increase_allowance_event: BTreeMap<String, String> = BTreeMap::new();

    expected_increase_allowance_event
        .insert("event_type".to_string(), "IncreaseAllowance".to_string());
    expected_increase_allowance_event
        .insert("cep18_package".to_string(), cep18_token_package.to_string());
    expected_increase_allowance_event.insert(
        "owner".to_string(),
        "Key::Account(58b891759929bd4ed5a9cce20b9d6e3c96a66c21386bed96040e17dd07b79fa7)"
            .to_string(),
    );
    expected_increase_allowance_event.insert(
        "spender".to_string(),
        "Key::Hash(2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a)".to_string(),
    );
    expected_increase_allowance_event.insert(
        "token_amount".to_string(),
        ((allowance_amount_1.saturating_mul(U256::from(2))).saturating_sub(allowance_amount_2))
            .to_string(),
    );
    expected_increase_allowance_event.insert("inc_by".to_string(), allowance_amount_1.to_string());

    assert_eq!(increase_allowance_event, expected_increase_allowance_event);
}
