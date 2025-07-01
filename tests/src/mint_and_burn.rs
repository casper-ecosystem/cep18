use crate::utility::{
    constants::{AMOUNT_1, AMOUNT_2, TOKEN_DECIMALS, TOKEN_NAME, TOKEN_SYMBOL, TOKEN_TOTAL_SUPPLY},
    installer_request_builders::{
        cep18_check_balance_of, cep18_check_total_supply, get_test_account, setup_with_args,
        TestContext,
    },
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_execution_engine::{engine_state::Error as CoreError, execution::ExecError};
use casper_types::{runtime_args, AddressableEntityHash, ApiError, Key, U256};
use cep18::{
    constants::{
        ADMIN_LIST, ARG_AMOUNT, ARG_DECIMALS, ARG_ENABLE_MINT_BURN, ARG_NAME, ARG_OWNER,
        ARG_SYMBOL, ARG_TOTAL_SUPPLY, ENTRY_POINT_BURN, ENTRY_POINT_CHANGE_SECURITY,
        ENTRY_POINT_MINT, MINTER_LIST, NONE_LIST,
    },
    error::Cep18Error,
};

#[test]
fn test_mint_and_burn_tokens() {
    let mint_amount = U256::one();

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
        ARG_ENABLE_MINT_BURN => true,
    });

    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");

    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();
    let mint_request_2 = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_2_key, ARG_AMOUNT => U256::from(AMOUNT_2)},
    )
    .build();
    builder.exec(mint_request_2).expect_success().commit();
    assert_eq!(
        cep18_check_balance_of(
            &mut builder,
            &cep18_contract_hash,
            Key::Account(*DEFAULT_ACCOUNT_ADDR)
        ),
        U256::from(TOKEN_TOTAL_SUPPLY),
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_contract_hash, account_user_1_key),
        U256::from(AMOUNT_1)
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_contract_hash, account_user_2_key),
        U256::from(AMOUNT_2)
    );
    let total_supply_before_mint = cep18_check_total_supply(&mut builder, &cep18_contract_hash);

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_contract_hash, account_user_1_key),
        U256::from(AMOUNT_1) + mint_amount,
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_contract_hash, account_user_2_key),
        U256::from(AMOUNT_2)
    );

    let total_supply_after_mint = cep18_check_total_supply(&mut builder, &cep18_contract_hash);
    assert_eq!(
        total_supply_after_mint,
        total_supply_before_mint + mint_amount,
    );
    let total_supply_before_burn = total_supply_after_mint;

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(
            &mut builder,
            &cep18_contract_hash,
            Key::Account(*DEFAULT_ACCOUNT_ADDR)
        ),
        U256::from(999999999),
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_contract_hash, account_user_2_key),
        U256::from(AMOUNT_2)
    );
    let total_supply_after_burn = cep18_check_total_supply(&mut builder, &cep18_contract_hash);
    assert_eq!(
        total_supply_after_burn,
        total_supply_before_burn - mint_amount,
    );

    assert_eq!(total_supply_after_burn, total_supply_before_mint);
}

#[test]
fn test_should_not_mint_above_limits() {
    let mint_amount = U256::MAX;

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
        ARG_ENABLE_MINT_BURN => true,
    });

    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();

    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");
    let mint_request_2 = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_2_key, ARG_AMOUNT => U256::from(AMOUNT_2)},
    )
    .build();
    builder.exec(mint_request_2).expect_success().commit();
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_contract_hash, account_user_1_key),
        U256::from(AMOUNT_1)
    );

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == Cep18Error::Overflow as u16),
        "Should not mint above limits, but instead: {error:?}",
    );
}

#[test]
fn test_should_not_burn_above_balance() {
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
        ARG_ENABLE_MINT_BURN => true,
    });

    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_AMOUNT => U256::from(TOKEN_TOTAL_SUPPLY)+1,
        },
    )
    .build();

    builder.exec(burn_request).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == Cep18Error::InsufficientBalance as u16),
        "{error:?}",
    );
}

#[test]
fn test_should_not_mint_or_burn_with_entrypoint_disabled() {
    let mint_amount = U256::one();

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
        ARG_ENABLE_MINT_BURN => false,
    });

    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60016),
        "{error:?}",
    );

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60016),
        "{error:?}",
    );
}

#[test]
fn test_security_no_rights() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let mint_amount = U256::one();

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
        ARG_ENABLE_MINT_BURN => true,
    });

    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1_account_hash,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60010),
        "{error:?}",
    );

    let passing_admin_mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder
        .exec(passing_admin_mint_request)
        .expect_success()
        .commit();

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1_account_hash,
        addressable_cep18_contract_hash,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).expect_success().commit();
}

#[test]
fn test_security_minter_rights() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let mint_amount = U256::one();

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
        ARG_ENABLE_MINT_BURN => true,
        MINTER_LIST => vec![account_user_1_key],
    });

    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1_account_hash,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit().expect_success();
}

#[test]
fn test_security_burner_rights() {
    let default_account_user_key = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let mint_amount = U256::one();

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
        ARG_ENABLE_MINT_BURN => true,
    });

    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1_account_hash,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60010),
        "{error:?}",
    );

    // mint by admin
    let working_mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => default_account_user_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(working_mint_request).commit().expect_success();

    // any user can burn
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_OWNER => default_account_user_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).commit().expect_success();
}

#[test]
fn test_change_security() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let mint_amount = U256::one();

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
        ARG_ENABLE_MINT_BURN => true,
        ADMIN_LIST => vec![account_user_1_key],
    });

    let addressable_cep18_contract_hash = AddressableEntityHash::new(cep18_contract_hash.value());
    let change_security_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1_account_hash,
        addressable_cep18_contract_hash,
        ENTRY_POINT_CHANGE_SECURITY,
        runtime_args! {
            NONE_LIST => vec![Key::Account(*DEFAULT_ACCOUNT_ADDR)],
        },
    )
    .build();

    builder
        .exec(change_security_request)
        .commit()
        .expect_success();

    // // account_user_1 was created before genesis and is not yet funded so fund it
    // fund_account(&mut builder, account_user_1);

    // let account_user_1_key = Key::Account(account_user_1);

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        addressable_cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_OWNER => account_user_1_key,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).expect_failure();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60010),
        "{error:?}",
    );
}
