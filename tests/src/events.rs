use crate::utility::{
    constants::{
        AMOUNT_1, CEP18_TEST_TOKEN_CONTRACT_NAME, TOKEN_DECIMALS, TOKEN_NAME, TOKEN_SYMBOL,
        TOKEN_TOTAL_SUPPLY,
    },
    installer_request_builders::{get_test_account, setup_with_args, TestContext},
    message_handlers::message_topic,
    support::get_event,
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_event_standard::EVENTS_DICT;
use casper_types::{
    contract_messages::Message, runtime_args, AddressableEntityHash, EntityAddr, U256,
};
use cep18::{
    constants::{
        ARG_AMOUNT, ARG_DECIMALS, ARG_ENABLE_MINT_BURN, ARG_EVENTS, ARG_EVENTS_MODE, ARG_NAME,
        ARG_OWNER, ARG_SYMBOL, ARG_TOTAL_SUPPLY, ENTRY_POINT_MINT,
    },
    events::Mint,
};

#[test]
fn should_have_have_no_events() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ARG_EVENTS_MODE => 0_u8,
        ARG_ENABLE_MINT_BURN => true,
    });

    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let addressable_cep18_token = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_token,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();

    let entity_addr = EntityAddr::SmartContract(addressable_cep18_token.value());
    let entity_with_named_keys = builder.get_named_keys(entity_addr);
    assert!(entity_with_named_keys.get(EVENTS_DICT).is_none());

    assert!(builder
        .message_topics(None, entity_addr)
        .unwrap()
        .is_empty());
}

#[test]
fn should_have_native_events() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ARG_EVENTS_MODE => 2_u8,
        ARG_ENABLE_MINT_BURN => true,
    });

    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let addressable_cep18_token = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_token,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();

    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .unwrap();
    let account_named_keys = account.named_keys();

    let cep18_token = account_named_keys
        .get(CEP18_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_entity_hash())
        .expect("should have contract hash");

    let entity_addr = EntityAddr::SmartContract(cep18_contract_hash.value());

    // events check
    let binding = builder.message_topics(None, entity_addr).unwrap();
    let (topic_name, message_topic_hash) = binding
        .iter()
        .last()
        .expect("should have at least one topic");

    assert_eq!(topic_name, &ARG_EVENTS.to_string());

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    assert_eq!(
        message_topic(&builder, &cep18_token, *message_topic_hash).message_count(),
        3
    );

    let exec_result = builder.get_exec_result_owned(3).unwrap();
    let messages = exec_result.messages();
    let mint_message = format!(
        "{{\"recipient\":\"{}\",\"amount\":\"1000000\"}}",
        account_user_1_account_hash.to_formatted_string()
    );
    let entity_addr = EntityAddr::SmartContract(cep18_token.value());
    let message = Message::new(
        entity_addr,
        mint_message.into(),
        ARG_EVENTS.to_string(),
        *message_topic_hash,
        2,
        2,
    );
    assert_eq!(messages, &vec![message]);
}

#[test]
fn should_have_ces_events() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ARG_EVENTS_MODE => 1_u8,
        ARG_ENABLE_MINT_BURN => true,
    });

    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let addressable_cep18_token = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_token,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();

    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .unwrap();
    let account_named_keys = account.named_keys();

    let cep18_token = account_named_keys
        .get(CEP18_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_entity_hash())
        .expect("should have contract hash");

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER=> account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let expected_event = Mint::new(account_user_1_key, U256::from(AMOUNT_1));
    let actual_event: Mint = get_event(&mut builder, &cep18_token, 1);
    assert_eq!(actual_event, expected_event, "Expected Mint event.");
}

#[test]
fn should_test_error_message_topic_on_mint_overflow() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ARG_EVENTS_MODE => 0_u8,
        ARG_ENABLE_MINT_BURN => true,
    });

    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let addressable_cep18_token = AddressableEntityHash::new(cep18_contract_hash.value());

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_token,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::MAX},
    )
    .build();

    builder.exec(mint_request).expect_failure().commit();

    let _ = builder.get_exec_result_owned(2).unwrap();
}
