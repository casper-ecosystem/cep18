# CasperLabs ERC20

Implementation of the ERC20 token standard for the Casper platform.
It enables developers to implement custom tokens in minutes.

## Usage
### Install
Make sure `wasm32-unknown-unknown` is installed.
```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

### Build Smart Contract
```
make build-contract
```

### Test
Test logic and smart contract.
```
make test
```

## Repository overview

### ERC20

The `erc20` crate contains the implementation of the ERC20 standard.

#### ERC20 as library
It can be used as a library to build custom tokens. The code structure allows
for easy entry points extensions and overrides.

##### Entry Point override example
The following code shows how to override the `transfer` method to alwasy mint
one additional token for a sender. 

```rust
struct Token {}

impl ContractContext for Token {}
impl ERC20 for Token {}

impl Token {
    ...
    fn transfer(&mut self, recipient: Key, amount: U256) {
        ERC20::mint(self, self.get_caller(), U256::one());
        ERC20::transfer(self, recipient, amount);
    }
}
```

#### ERC20 Vanilla Contract
The library comes with a vanilla implementation of the ERC20 contract that is
ready to use. It is implemented in `erc20/bin/erc20_token.rs` and after 
compilation the `erc20-token.wasm` file is produced.

### ERC20 Tests
The `erc20-tests` crate implements multiple integration test scenarios that
check the compatibility with the ERC20 standard.

Tests provide the `ERC20Instance` struct that can be reused in larger smart
contract projects with multiple ERC20 tokens and other smart contracts
to interact with the instance of an ERC20 token.

Tests are implemented in `erc20-tests/src/erc20_tests.rs`.

### Utils

The repository contains 3 utility crates:

* `utils/test-env`
* `utils/contract-utils`
* `utils/contract-interface`

The utility code after review and adoption should be moved to a separate repo
and eventually be added to `casper-contract` and `casper-engine-test-support`. 
The functionality provided by the utils crates releases a smart contract developer
from writing boilerplate code and allows for **rapid development**.

#### Test Env Crate
`utils/test-env` is a small library written on top of 
`casper-engine-test-support`. It provides two structs:

* `TestEnv` wraps `TestContext` and provides user accounts with initial
  CSPR balances. It is implemented using `Arc<Mutex<...>>` so it can
  be copied, especial between `TestContract` instances.
* `TestContract` wraps an instance of `TestEnv` and simplifies calling
  contracts and reading named keys and dictionaries.

##### Test Example
```rust
struct Token(TestContract);

impl Token {
    pub fn new(env: &TestEnv, sender: Sender) -> Token {
        Token(TestContract::new(env, "token.wasm", "token_name", sender, runtime_args! {
            "initial_supply" => U256::from(1000)
        }))
    }

    pub fn transfer(&self, sender: Sender, recipient: AccountHash, amount: u64) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    pub fn balance_of(&self, account: AccountHash) -> u64 {
        self.0
            .query_dictionary("balances", account.to_string())
            .unwrap_or_default()
    }
}

#[test]
fn test_multiple_tokens() {
    // Prepare the env and users.
    let env = TestEnv::new();
    let user1 = env.next_user();
    let user2 = env.next_user();
    
    // Deploy multiple instances of the same contract
    // agains a single virtual machine.
    let token1 = Token::new(&env, Sender(user1));
    let token2 = Token::new(&env, Sender(user2));

    // Transfer tokens.
    let amount = 10;
    token1.transfer(Sender(user1), user2, amount);
    assert_eq!(token1.balance_of(user1), amount);
}
```

#### Contract Utils Crate
`utils/contract-utils` contains common building blocks for writing smart
contracts:
* `contract_context.rs` provides the `ContractContext` trait that has 
  `get_caller` and `self_addr` methods.
* `data.rs` provides helper methods to work with dictionaries and named
  keys.
* `admin_control.rs` provides the `AdminControl` trait to support admin
  list functionality.

#### Contract Interface Crate
`utils/contract-interface` introduces the `contract_interface` procedural
macro that generates entry points definitions, `no_mangle` functions
and the call method.

Consider the following example of a simple counter contract.
```rust
#![no_main]
#![no_std]

extern crate alloc;

use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use contract_interface::contract_interface;
use contract_utils::{get_key, set_key};

struct Counter {}

impl Counter {
    fn constructor(&mut self) {
        set_key("counter", 0u64);
    }

    fn increment(&mut self, step: u64) {
        set_key("counter", self.get_value() + step);
    }

    fn get_value(&mut self) -> u64 {
        get_key("counter").unwrap_or_revert()
    }
}

#[contract_interface(Counter)]
trait CounterInterface {
    fn constructor(&mut self);
    fn increment(&mut self, step: u64);
    fn get_value(&mut self) -> u64;
}
```

It expands into.
```rust
#![no_main]
#![no_std]

extern crate alloc;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use contract_interface::contract_interface;
use contract_utils::{get_key, set_key};

struct Counter {}

impl Counter {
    fn constructor(&mut self) {
        set_key("counter", 0u64);
    }

    fn increment(&mut self, step: u64) {
        set_key("counter", self.get_value() + step);
    }

    fn get_value(&mut self) -> u64 {
        get_key("counter").unwrap_or_revert()
    }
}

trait CounterInterface {
    fn constructor(&mut self);
    fn increment(&mut self, step: u64);
    fn get_value(&mut self) -> u64;
}

#[no_mangle]
fn constructor() {
    Counter {}.constructor();
}

#[no_mangle]
fn increment() {
    let step: u64 = casper_contract::contract_api::runtime::get_named_arg("step");
    Counter {}.increment(step);
}

#[no_mangle]
fn get_value() {
    use casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let ret: u64 = Counter {}.get_value();
    casper_contract::contract_api::runtime::ret(
        casper_types::CLValue::from_t(ret).unwrap_or_revert(),
    );
}

fn get_entry_points() -> casper_types::EntryPoints {
    use casper_types::CLTyped;
    let mut entry_points = casper_types::EntryPoints::new();
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "constructor",
        ::alloc::vec::Vec::new(),
        <()>::cl_type(),
        casper_types::EntryPointAccess::Groups(<[_]>::into_vec(box [casper_types::Group::new(
            "constructor",
        )])),
        casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "increment",
        vec![casper_types::Parameter::new("step", <u64>::cl_type())],
        <()>::cl_type(),
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "get_value",
        ::alloc::vec::Vec::new(),
        <u64>::cl_type(),
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    use casper_contract::contract_api::{storage, runtime};
    use casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let mut constructor_args = casper_types::RuntimeArgs::new();
    let constructor_access: casper_types::URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);
    let mut urefs = alloc::collections::BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    runtime::put_key(&alloc::format!("{}_package_hash", contract_name), package_hash.into());
    runtime::put_key(&alloc::format!("{}_package_hash_wrapped", contract_name), storage::new_uref(package_hash).into());
    runtime::put_key(&alloc::format!("{}_contract_hash", contract_name), contract_hash.into());
    runtime::put_key(&alloc::format!("{}_contract_hash_wrapped", contract_name), storage::new_uref(contract_hash).into());
    runtime::put_key(&alloc::format!("{}_package_access_token", contract_name), access_token.into());
}
```
