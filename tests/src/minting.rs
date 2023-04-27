use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        AMOUNT, ARG_AMOUNT, ARG_OWNER, ERROR_OVERFLOW, METHOD_BURN, METHOD_MINT, OWNER,
        TOKEN_OWNER_ADDRESS_1, TOKEN_OWNER_ADDRESS_2, TOKEN_OWNER_AMOUNT_1, TOKEN_OWNER_AMOUNT_2,
        TOKEN_TOTAL_SUPPLY,
    },
    installer_request_builders::{
        cep18_check_balance_of, cep18_check_total_supply, setup, TestContext,
    },
};

use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

#[test]
fn test_mint_and_burn_tokens() {
    let mint_amount = U256::one();

    let (mut builder, TestContext { cep18_token, .. }) = setup();
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
}

#[test]
fn test_should_not_mint_above_limits() {
    let mint_amount = U256::MAX;

    let (mut builder, TestContext { cep18_token, .. }) = setup();
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
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );

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

    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_OVERFLOW),
        "{:?}",
        error
    );
}
