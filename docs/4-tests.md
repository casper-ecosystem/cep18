# Testing Framework for CEP-18

The testing framework in this tutorial uses the [Casper engine test support](https://crates.io/crates/casper-engine-test-support) crate for testing the contract implementation against the Casper execution environment.

The following section reviews the [GitHub testing folder](https://github.com/casper-ecosystem/cep18/tree/master/tests), which creates a testing framework for the Casper [Fungible Token](https://github.com/casper-ecosystem/cep18) project. You can find more details about testing Casper contracts [here](https://docs.casper.network/developers/writing-onchain-code/testing-contracts/).

The following is an example of a complete test expecting a failed transfer:

```rust
#[should_panic(expected = "ApiError::User(65534) [131070]")]
#[test]
fn should_not_transfer_with_insufficient_balance() {
    let mut fixture = TestFixture::install_contract();

    let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
    assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

    fixture.transfer(
        Key::from(fixture.bob),
        initial_ali_balance + U256::one(),
        fixture.ali,
    );
}
```

To build and run the tests, issue the following command in the project folder, [cep18](https://github.com/casper-ecosystem/cep18):

```bash
make test
```

The project contains a [Makefile](https://github.com/casper-ecosystem/cep18/blob/dev/Makefile), which is a custom build script that compiles the contract before running tests in _release_ mode. Then, the script copies the `contract.wasm` file to the [tests/wasm](https://github.com/casper-ecosystem/cep18/tree/master/tests/wasm) directory. In practice, you only need to run the `make test` command during development.

## Configuring the Test Package

In this project, we define a `tests` package using the [tests/Cargo.toml](https://github.com/casper-ecosystem/cep18/blob/dev/tests/Cargo.toml) file.

```bash
[package]
name = "tests"
version = "1.0.0"
...

[dependencies]
casper-types = "2.0.0"
casper-engine-test-support = "4.0.0"
casper-execution-engine = "4.0.0"
once_cell = "1.16.0"

[lib]
name = "tests"
...
```

## Testing Logic

In Github, you will find an [example](https://github.com/casper-ecosystem/cep18/tree/dev/cep18) containing a Casper Fungible Token [contract](https://github.com/casper-ecosystem/cep18/blob/dev/cep18/src/main.rs) implementation with the corresponding [tests](https://github.com/casper-ecosystem/cep18/tree/dev/tests/src). The tests follow this sequence:

- [Step 1](#setting-up-the-testing-context) - Specify the starting state of the blockchain.
- [Step 2](#deploying-the-contract) - Deploy the compiled contract to the blockchain and query it.
- [Step 3](#invoking-contract-entrypoints) - Create additional deploys for calling each of the entrypoints in the contract.

The [TestFixture](https://github.com/casper-ecosystem/cep18/blob/master/example/cep18-tests/src/test_fixture/test_fixture.rs) accomplishes these steps by simulating a real-world deploy that stores the contract on the blockchain and then invoking the contract's entrypoints.

### Setting up the Testing Context

The code in the [utility directory](https://github.com/casper-ecosystem/cep18/tree/dev/tests/src/utility) initializes the blockchain's [global state](https://docs.casper.network/concepts/glossary/G/#global-state) with all the data and entrypoints the smart contract needs.

Expand the example below to see a subset of the required constants for this project. The testing framework defines constants via the [`constants.rs`](https://github.com/casper-ecosystem/cep18/blob/dev/tests/src/utility/constants.rs) file within the `utility` directory. For the most up-to-date version of the code, visit [GitHub](https://github.com/casper-ecosystem/cep18).

<details>
<summary>Example of required constants</summary>

```rust
// File https://github.com/casper-ecosystem/cep18/blob/dev/tests/src/utility/installer_request_builders.rs

use casper_engine_test_support::{
    ExecuteRequestBuilder, LmdbWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    MINIMUM_ACCOUNT_CREATION_BALANCE, LOCAL_GENESIS_REQUEST,
};
use casper_execution_engine::core::engine_state::ExecuteRequest;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, system::mint, CLTyped, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};

use crate::utility::constants::{
    AMOUNT_ALLOWANCE_1, AMOUNT_ALLOWANCE_2, TOTAL_SUPPLY_KEY, AMOUNT_TRANSFER_1, AMOUNT_TRANSFER_2,
};

use super::constants::{
    ACCOUNT_1_ADDR, ACCOUNT_2_ADDR, ARG_ADDRESS, ARG_AMOUNT, ARG_DECIMALS, ARG_NAME, ARG_OWNER, ARG_RECIPIENT, ARG_SPENDER, ARG_SYMBOL, ARG_TOKEN_CONTRACT, ARG_TOTAL_SUPPLY, CEP18_CONTRACT_WASM, CEP18_TEST_CONTRACT_KEY, CEP18_TEST_CONTRACT_WASM, CEP18_TOKEN_CONTRACT_KEY, CHECK_ALLOWANCE_OF_ENTRYPOINT, CHECK_BALANCE_OF_ENTRYPOINT,CHECK_TOTAL_SUPPLY_ENTRYPOINT, METHOD_APPROVE, METHOD_APPROVE_AS_STORED_CONTRACT,METHOD_TRANSFER, METHOD_TRANSFER_AS_STORED_CONTRACT, RESULT_KEY, TOKEN_DECIMALS, TOKEN_NAME, TOKEN_SYMBOL, TOKEN_TOTAL_SUPPLY,
};
```

</details>

### Installing the Contract

The next step is to define a struct that has its own virtual machine (VM) instance and implements the Fungible Token entrypoints. This struct holds a `TestContext` of its own. The _contract_hash_ and the _session_code_ won’t change after the contract is deployed, so it is good to keep them handy.

This code snippet builds the context and includes the compiled contract _.wasm_ binary being tested. The `TestContext` struct creates a new instance of the `cep18_token` with several test accounts.

**Note**: These accounts have a positive initial balance.

The full and most recent code implementation is available on [GitHub](https://github.com/casper-ecosystem/cep18/blob/dev/tests/src/utility/installer_request_builders.rs).

<details>
<summary>Example of a CEP-18 token in a test</summary>

```rust
// File https://github.com/casper-ecosystem/cep18/blob/dev/tests/src/utility/installer_request_builders.rs

// Creating the `TestContext` struct.

pub(crate) struct TestContext {
pub(crate) cep18_token: ContractHash,
pub(crate) cep18_test_contract_package: ContractPackageHash,
}

// Setting up the test instance of CEP-18.

pub(crate) fn setup() -> (LmdbWasmTestBuilder, TestContext) {
    setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
    })
}

// Establishing test accounts.

pub(crate) fn setup_with_args(install_args: RuntimeArgs) -> (LmdbWasmTestBuilder, TestContext) {
    let mut builder = LmdbWasmTestBuilder::default();
    builder.run_genesis(&LOCAL_GENESIS_REQUEST);

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

    // Installing the test version of CEP-18 with the default account.

    let install_request_1 =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CEP18_CONTRACT_WASM, install_args)
            .build();

    let install_request_2 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_TEST_CONTRACT_WASM,
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

    let cep18_test_contract_package = account
        .named_keys()
        .get(CEP18_TEST_CONTRACT_KEY)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    let test_context = TestContext {
        cep18_token,
        cep18_test_contract_package,
    };

    (builder, test_context)
}
```

</details>

### Creating Helper Functions

The previous step has simulated sending a real deploy on the network. The next code snippet in `installer_request_builders.rs` defines helper functions that will be used throughout the testing framework:

- `cep18_check_total_supply` - A function for testing the total supply of the CEP-18 contract instance.
- `cep18_check_balance_of` - A function for checking an account's balance of CEP-18 tokens.
- `cep18_check_allowance_of` - A function for checking an account's spending allowance from another account's balance.

These are followed by functions that check specific aspects of the CEP-18 contract. These include `test_cep18_transfer`, `make_cep18_approve_request` and `test_approve_for`.

The following code snippet is an example function that tests the ability to transfer CEP-18 tokens from the default address to the two other addresses established in [contract installation](#installing-the-contract-deploying-the-contract):

<details>
<summary>Example helper function</summary>

```rust
// File https://github.com/casper-ecosystem/cep18/blob/dev/tests/src/utility/installer_request_builders.rs

pub(crate) fn test_cep18_transfer(
    builder: &mut LmdbWasmTestBuilder,
    test_context: &TestContext,
    sender1: Key,
    recipient1: Key,
    sender2: Key,
    recipient2: Key) {
    let TestContext { cep18_token, .. } = test_context;

    // Defining the amount to be transferred to each account.

    let transfer_amount_1 = U256::from(AMOUNT_TRANSFER_1);
    let transfer_amount_2 = U256::from(AMOUNT_TRANSFER_2);

    // Checking the pre-existing balances of the default address and the two receiving addresses.

    let sender_balance_before = cep18_check_balance_of(builder, cep18_token, sender1);
    assert_ne!(sender_balance_before, U256::zero());

    let account_1_balance_before = cep18_check_balance_of(builder, cep18_token, recipient1);
    assert_eq!(account_1_balance_before, U256::zero());

    let account_2_balance_before = cep18_check_balance_of(builder, cep18_token, recipient1);
    assert_eq!(account_2_balance_before, U256::zero());

    // Creating the first transfer request.

    let token_transfer_request_1 =
        make_cep18_transfer_request(sender1, cep18_token, recipient1, transfer_amount_1);

    builder
        .exec(token_transfer_request_1)
        .expect_success()
        .commit();

    // Checking the prior balance against the new balance to ensure the transfer occurred correctly.

    let account_1_balance_after = cep18_check_balance_of(builder, cep18_token, recipient1);
    assert_eq!(account_1_balance_after, transfer_amount_1);
    let account_1_balance_before = account_1_balance_after;

    let sender_balance_after = cep18_check_balance_of(builder, cep18_token, sender1);
    assert_eq!(
        sender_balance_after,
        sender_balance_before - transfer_amount_1
    );
    let sender_balance_before = sender_balance_after;

    // Creating the second transfer request.

    let token_transfer_request_2 =
        make_cep18_transfer_request(sender2, cep18_token, recipient2, transfer_amount_2);

    builder
        .exec(token_transfer_request_2)
        .expect_success()
        .commit();

    // Checking prior balances against new balances.

    let sender_balance_after = cep18_check_balance_of(builder, cep18_token, sender1);
    assert_eq!(sender_balance_after, sender_balance_before);

    let account_1_balance_after = cep18_check_balance_of(builder, cep18_token, recipient1);
    assert!(account_1_balance_after < account_1_balance_before);
    assert_eq!(
        account_1_balance_after,
        transfer_amount_1 - transfer_amount_2
    );

    let account_2_balance_after = cep18_check_balance_of(builder, cep18_token, recipient2);
    assert_eq!(account_2_balance_after, transfer_amount_2);
}
```

</details>

## Creating Unit Tests

Within this testing context, the [`tests` directory](https://github.com/casper-ecosystem/cep18/tree/dev/tests/src) includes a variety of unit tests, which verify the contract code by invoking the functions defined in the [installer_request_builders.rs](https://github.com/casper-ecosystem/cep18/blob/dev/tests/src/utility/installer_request_builders.rs) file.

The example below shows one of the tests. Visit [GitHub](https://github.com/casper-ecosystem/cep18/tree/dev/tests/src) to find all the available tests.

<details>
<summary>Example test querying token properties</summary>

```rust
// File https://github.com/casper-ecosystem/cep18/blob/dev/tests/src/install.rs

use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{Key, U256};

use crate::utility::{
    constants::{
        ALLOWANCES_KEY, BALANCES_KEY, DECIMALS_KEY, NAME_KEY, SYMBOL_KEY, TOKEN_DECIMALS,
        TOKEN_NAME, TOKEN_SYMBOL, TOKEN_TOTAL_SUPPLY, TOTAL_SUPPLY_KEY,
    },
    installer_request_builders::{
        cep18_check_balance_of, invert_cep18_address, setup, TestContext,
    },
};

#[test]
fn should_have_queryable_properties() {
    let (mut builder, TestContext { cep18_token, .. }) = setup();

    let name: String = builder.get_value(cep18_token, NAME_KEY);
    assert_eq!(name, TOKEN_NAME);

    let symbol: String = builder.get_value(cep18_token, SYMBOL_KEY);
    assert_eq!(symbol, TOKEN_SYMBOL);

    let decimals: u8 = builder.get_value(cep18_token, DECIMALS_KEY);
    assert_eq!(decimals, TOKEN_DECIMALS);

    let total_supply: U256 = builder.get_value(cep18_token, TOTAL_SUPPLY_KEY);
    assert_eq!(total_supply, U256::from(TOKEN_TOTAL_SUPPLY));

    let owner_key = Key::Account(*DEFAULT_ACCOUNT_ADDR);

    let owner_balance = cep18_check_balance_of(&mut builder, &cep18_token, owner_key);
    assert_eq!(owner_balance, total_supply);

    let contract_balance =
        cep18_check_balance_of(&mut builder, &cep18_token, Key::Hash(cep18_token.value()));
    assert_eq!(contract_balance, U256::zero());

    // Ensures that Account and Contract ownership is respected and we're not keying ownership under
    // the raw bytes regardless of variant.
    let inverted_owner_key = invert_cep18_address(owner_key);
    let inverted_owner_balance =
        cep18_check_balance_of(&mut builder, &cep18_token, inverted_owner_key);
    assert_eq!(inverted_owner_balance, U256::zero());
}
```

</details>

## Running the Tests

The [lib.rs](https://github.com/casper-ecosystem/cep18/blob/dev/tests/src/lib.rs) file is configured to run the example integration tests via the `make test` command:

```rust
#[cfg(test)]
mod allowance;
#[cfg(test)]
mod install;
#[cfg(test)]
mod mint_and_burn;
#[cfg(test)]
mod transfer;
#[cfg(test)]
mod utility;
```

To run the tests, navigate to the parent [cep18 directory](https://github.com/casper-ecosystem/cep18) and run the command:

```bash
make test
```

This example uses `bash`. If you are using a Rust IDE, you need to configure it to run the tests.
