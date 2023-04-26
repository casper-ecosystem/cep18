use casper_engine_test_support::{DEFAULT_ACCOUNT_ADDR, ExecuteRequestBuilder};
use casper_types::{runtime_args, RuntimeArgs, Key, U256, ApiError};

use crate::utility::{constants::{TOKEN_TOTAL_SUPPLY, ARG_AMOUNT, ARG_RECIPIENT, ACCOUNT_1_ADDR, METHOD_TRANSFER, TOTAL_SUPPLY_KEY, ERROR_INSUFFICIENT_BALANCE, ALLOWANCE_AMOUNT_1, ARG_SPENDER, ARG_OWNER, METHOD_APPROVE, METHOD_TRANSFER_FROM, ARG_TOKEN_CONTRACT, METHOD_FROM_AS_STORED_CONTRACT, TRANSFER_AMOUNT_1, ACCOUNT_2_ADDR, TOKEN_OWNER_ADDRESS_1, TOKEN_OWNER_AMOUNT_1, TOKEN_OWNER_ADDRESS_2, TOKEN_OWNER_AMOUNT_2, METHOD_MINT, METHOD_BURN, ERROR_OVERFLOW}, installer_request_builders::{TestContext, setup, cep18_check_balance_of, cep18_check_allowance_of, make_cep18_transfer_request, make_cep18_approve_request, test_cep18_transfer, cep18_check_total_supply}};

use casper_execution_engine::core::{
    engine_state::{Error as CoreError, ExecuteRequest},
    execution::Error as ExecError,
};


#[test]
fn test_mint_and_burn_tokens() {
    let mint_amount = U256::one();

    let (mut builder, TestContext { test_contract, .. }) = setup();
    assert_eq!(
        cep18_check_balance_of(
            &mut builder,
            &test_contract,
            Key::Account(*DEFAULT_ACCOUNT_ADDR)
        ),
        U256::from(TOKEN_TOTAL_SUPPLY),
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );
    let total_supply_before_mint = cep18_check_total_supply(&mut builder, &test_contract);

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        test_contract,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1) + mint_amount,
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );

    let total_supply_after_mint = cep18_check_total_supply(&mut builder, &test_contract);
    assert_eq!(
        total_supply_after_mint,
        total_supply_before_mint + mint_amount,
    );
    let total_supply_before_burn = total_supply_after_mint;

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        test_contract,
        METHOD_BURN,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1),
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );
    let total_supply_after_burn = cep18_check_total_supply(&mut builder, &test_contract);
    assert_eq!(
        total_supply_after_burn,
        total_supply_before_burn - mint_amount,
    );

    assert_eq!(total_supply_after_burn, total_supply_before_mint);
}

#[test]
fn test_should_not_mint_above_limits() {
    let mint_amount = U256::MAX;

    let (mut builder, TestContext { test_contract, .. }) = setup();
    assert_eq!(
        cep18_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        test_contract,
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
