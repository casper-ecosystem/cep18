use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ALLOWANCE_AMOUNT_1, ARG_AMOUNT, ARG_OWNER, ARG_RECIPIENT, ARG_SPENDER,
        ERROR_INSUFFICIENT_ALLOWANCE, METHOD_APPROVE, METHOD_TRANSFER_FROM,
    },
    installer_request_builders::{cep18_check_allowance_of, setup, test_approve_for, TestContext},
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
