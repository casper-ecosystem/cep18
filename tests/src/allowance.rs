use std::collections::BTreeMap;

use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ALLOWANCE_AMOUNT_1, ALLOWANCE_AMOUNT_2, ARG_AMOUNT, ARG_OWNER,
        ARG_RECIPIENT, ARG_SPENDER, DECREASE_ALLOWANCE, ERROR_INSUFFICIENT_ALLOWANCE,
        INCREASE_ALLOWANCE, METHOD_APPROVE, METHOD_TRANSFER_FROM,
    },
    installer_request_builders::{
        cep18_check_allowance_of, get_dictionary_value_from_key, make_cep18_approve_request, setup,
        test_approve_for, TestContext,
    },
};
use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

#[test]
fn should_approve_funds_contract_to_account() {
    let (mut builder, test_context) = setup();
    let TestContext {
        cep18_test_contract,
        ..
    } = test_context;

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Hash(cep18_test_contract.value()),
        Key::Hash(cep18_test_contract.value()),
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
}

#[test]
fn should_approve_funds_contract_to_contract() {
    let (mut builder, test_context) = setup();
    let TestContext {
        cep18_test_contract,
        ..
    } = test_context;

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Hash(cep18_test_contract.value()),
        Key::Hash(cep18_test_contract.value()),
        Key::Hash([42; 32]),
    );
}

#[test]
fn should_approve_funds_account_to_account() {
    let (mut builder, test_context) = setup();

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        Key::Account(*ACCOUNT_1_ADDR),
    );
}

#[test]
fn should_approve_funds_account_to_contract() {
    let (mut builder, test_context) = setup();
    test_approve_for(
        &mut builder,
        &test_context,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        Key::Hash([42; 32]),
    );
}

#[test]
fn should_not_transfer_from_without_enough_allowance() {
    let (mut builder, TestContext { cep18_token, .. }) = setup();

    let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
    let transfer_from_amount_1 = allowance_amount_1 + U256::one();

    let sender = *DEFAULT_ACCOUNT_ADDR;
    let owner = sender;
    let recipient = *ACCOUNT_1_ADDR;

    let cep18_approve_args = runtime_args! {
        ARG_OWNER => Key::Account(owner),
        ARG_SPENDER => Key::Account(recipient),
        ARG_AMOUNT => allowance_amount_1,
    };
    let cep18_transfer_from_args = runtime_args! {
        ARG_OWNER => Key::Account(owner),
        ARG_RECIPIENT => Key::Account(recipient),
        ARG_AMOUNT => transfer_from_amount_1,
    };

    let spender_allowance_before =
        cep18_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(recipient));
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        sender,
        cep18_token,
        METHOD_APPROVE,
        cep18_approve_args,
    )
    .build();

    let transfer_from_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        sender,
        cep18_token,
        METHOD_TRANSFER_FROM,
        cep18_transfer_from_args,
    )
    .build();

    builder.exec(approve_request_1).expect_success().commit();

    let account_1_allowance_after =
        cep18_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(recipient));
    assert_eq!(account_1_allowance_after, allowance_amount_1);

    builder.exec(transfer_from_request_1).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_INSUFFICIENT_ALLOWANCE),
        "{:?}",
        error
    );
}

#[test]
fn test_decrease_allowance() {
    let (
        mut builder,
        TestContext {
            cep18_token,
            cep18_token_package,
            ..
        },
    ) = setup();
    let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let owner = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let spender = Key::Hash([42; 32]);
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

    let account_1_allowance_after = cep18_check_allowance_of(&mut builder, owner, spender);

    assert_eq!(account_1_allowance_after, allowance_amount_1);

    builder
        .exec(decrease_allowance_request)
        .expect_success()
        .commit();

    let account_1_allowance_after_decrease = cep18_check_allowance_of(&mut builder, owner, spender);

    assert_eq!(
        account_1_allowance_after_decrease,
        allowance_amount_1 - allowance_amount_2
    );

    builder
        .exec(increase_allowance_request)
        .expect_success()
        .commit();

    let account_1_allowance_after_increase = cep18_check_allowance_of(&mut builder, owner, spender);

    assert_eq!(
        account_1_allowance_after_increase,
        (allowance_amount_1 * 2) - allowance_amount_2
    );

    // approve event

    let approve_event = get_dictionary_value_from_key::<BTreeMap<String, String>>(
        &builder,
        &cep18_token.into(),
        "events",
        "0",
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
        "1",
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
        "2",
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
