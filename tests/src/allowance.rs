use crate::utility::{
    constants::{AMOUNT_ALLOWANCE_1, AMOUNT_ALLOWANCE_2},
    installer_request_builders::{
        cep18_check_allowance_of, get_test_account, make_cep18_approve_request, setup,
        test_approve_for, TestContext,
    },
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_KEY};
use casper_execution_engine::{engine_state::Error as CoreError, execution::ExecError};
use casper_types::{runtime_args, AddressableEntityHash, ApiError, Key, U256};
use cep18::{
    constants::{
        ARG_AMOUNT, ARG_OWNER, ARG_RECIPIENT, ARG_SPENDER, ENTRY_POINT_APPROVE,
        ENTRY_POINT_DECREASE_ALLOWANCE, ENTRY_POINT_INCREASE_ALLOWANCE, ENTRY_POINT_TRANSFER_FROM,
    },
    error::Cep18Error,
};

#[test]
fn should_approve_funds_contract_to_account() {
    let (mut builder, test_context) = setup();
    let TestContext {
        cep18_test_contract_package,
        ..
    } = test_context;

    let (default_account_user_key, _, _) = get_test_account("ACCOUNT_USER_0");

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Hash(cep18_test_contract_package.value()),
        Key::Hash(cep18_test_contract_package.value()),
        default_account_user_key,
    );
}

#[test]
fn should_approve_funds_contract_to_contract() {
    let (mut builder, test_context) = setup();
    let TestContext {
        cep18_test_contract_package,
        ..
    } = test_context;

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Hash(cep18_test_contract_package.value()),
        Key::Hash(cep18_test_contract_package.value()),
        Key::Hash([42; 32]),
    );
}

#[test]
fn should_approve_funds_account_to_account() {
    let (mut builder, test_context) = setup();

    let (default_account_user_key, _, _) = get_test_account("ACCOUNT_USER_0");
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    test_approve_for(
        &mut builder,
        &test_context,
        default_account_user_key,
        default_account_user_key,
        account_user_1_key,
    );
}

#[test]
fn should_approve_funds_account_to_contract() {
    let (mut builder, test_context) = setup();

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Account(*DEFAULT_ACCOUNT_KEY),
        Key::Account(*DEFAULT_ACCOUNT_KEY),
        Key::Hash([42; 32]),
    );
}

#[test]
fn should_not_transfer_from_without_enough_allowance() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup();

    let (_, sender, _) = get_test_account("ACCOUNT_USER_0");
    let (recipient_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());

    let allowance_amount_1 = U256::from(AMOUNT_ALLOWANCE_1);
    let transfer_from_amount_1 = allowance_amount_1 + U256::one();

    let sender_key = Key::Account(*DEFAULT_ACCOUNT_KEY);
    let owner_key = sender_key;

    let cep18_approve_args = runtime_args! {
        ARG_OWNER => owner_key,
        ARG_SPENDER => recipient_key,
        ARG_AMOUNT => allowance_amount_1,
    };
    let cep18_transfer_from_args = runtime_args! {
        ARG_OWNER => owner_key,
        ARG_RECIPIENT => recipient_key,
        ARG_AMOUNT => transfer_from_amount_1,
    };

    let spender_allowance_before = cep18_check_allowance_of(&mut builder, owner_key, recipient_key);
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        sender,
        addressable_cep18_contract_hash,
        ENTRY_POINT_APPROVE,
        cep18_approve_args,
    )
    .build();

    let transfer_from_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        sender,
        addressable_cep18_contract_hash,
        ENTRY_POINT_TRANSFER_FROM,
        cep18_transfer_from_args,
    )
    .build();

    builder.exec(approve_request_1).expect_success().commit();

    let account_1_allowance_after =
        cep18_check_allowance_of(&mut builder, owner_key, recipient_key);
    assert_eq!(account_1_allowance_after, allowance_amount_1);

    builder.exec(transfer_from_request_1).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == Cep18Error::InsufficientAllowance as u16),
        "{error:?}"
    );
}

#[test]
fn test_decrease_allowance() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup();

    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());

    let (owner, owner_account_hash, _) = get_test_account("ACCOUNT_USER_0");
    let spender = Key::Hash([42; 32]);
    let spender_key = Key::Hash([42; 32]);

    let allowance_amount_1 = U256::from(AMOUNT_ALLOWANCE_1);
    let allowance_amount_2 = U256::from(AMOUNT_ALLOWANCE_2);

    let spender_allowance_before = cep18_check_allowance_of(&mut builder, owner, spender_key);
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request =
        make_cep18_approve_request(owner, &cep18_contract_hash, spender, allowance_amount_1);
    let decrease_allowance_request = ExecuteRequestBuilder::contract_call_by_hash(
        owner_account_hash,
        addressable_cep18_contract_hash,
        ENTRY_POINT_DECREASE_ALLOWANCE,
        runtime_args! {
            ARG_SPENDER => spender,
            ARG_AMOUNT => allowance_amount_2,
        },
    )
    .build();
    let increase_allowance_request = ExecuteRequestBuilder::contract_call_by_hash(
        owner_account_hash,
        addressable_cep18_contract_hash,
        ENTRY_POINT_INCREASE_ALLOWANCE,
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
}
