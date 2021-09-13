#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
};

use casper_contract::{
    self,
    contract_api::{runtime, storage},
};
use casper_erc20::{
    constants::{
        AMOUNT_RUNTIME_ARG_NAME, APPROVE_ENTRY_POINT_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        TRANSFER_ENTRY_POINT_NAME, TRANSFER_FROM_ENTRY_POINT_NAME,
    },
    Address,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, CLTyped, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, U256,
};

const CHECK_TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "check_total_supply";
const CHECK_BALANCE_OF_ENTRY_POINT_NAME: &str = "check_balance_of";
const TRANSFER_AS_STORED_CONTRACT_ENTRY_POINT_NAME: &str = "transfer_as_stored_contract";
const APPROVE_AS_STORED_CONTRACT_ENTRY_POINT_NAME: &str = "approve_as_stored_contract";
const TRANSFER_FROM_AS_STORED_CONTRACT_ENTRY_POINT_NAME: &str = "transfer_from_as_stored_contract";
const CHECK_ALLOWANCE_OF_ENTRY_POINT_NAME: &str = "check_allowance_of";
const TOKEN_CONTRACT_RUNTIME_ARG_NAME: &str = "token_contract";
const ADDRESS_RUNTIME_ARG_NAME: &str = "address";
const OWNER_RUNTIME_ARG_NAME: &str = "owner";
const SPENDER_RUNTIME_ARG_NAME: &str = "spender";
const RESULT_KEY: &str = "result";
const ERC20_TEST_CALL_KEY: &str = "erc20_test_call";

fn store_result<T: CLTyped + ToBytes>(result: T) {
    match runtime::get_key(RESULT_KEY) {
        Some(Key::URef(uref)) => storage::write(uref, result),
        Some(_) => unreachable!(),
        None => {
            let new_uref = storage::new_uref(result);
            runtime::put_key(RESULT_KEY, new_uref.into());
        }
    }
}

#[no_mangle]
extern "C" fn check_total_supply() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let total_supply: U256 = runtime::call_contract(
        token_contract,
        casper_erc20::constants::TOTAL_SUPPLY_ENTRY_POINT_NAME,
        RuntimeArgs::default(),
    );
    store_result(total_supply);
}

#[no_mangle]
extern "C" fn check_balance_of() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);

    let balance_args = runtime_args! {
        casper_erc20::constants::ADDRESS_RUNTIME_ARG_NAME => address,
    };
    let result: U256 = runtime::call_contract(
        token_contract,
        casper_erc20::constants::BALANCE_OF_ENTRY_POINT_NAME,
        balance_args,
    );

    store_result(result);
}

#[no_mangle]
extern "C" fn check_allowance_of() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);

    let allowance_args = runtime_args! {
        casper_erc20::constants::OWNER_RUNTIME_ARG_NAME => owner,
        casper_erc20::constants::SPENDER_RUNTIME_ARG_NAME => spender,
    };
    let result: U256 = runtime::call_contract(
        token_contract,
        casper_erc20::constants::ALLOWANCE_ENTRY_POINT_NAME,
        allowance_args,
    );

    store_result(result);
}

#[no_mangle]
extern "C" fn transfer_as_stored_contract() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    let transfer_args = runtime_args! {
        RECIPIENT_RUNTIME_ARG_NAME => recipient,
        AMOUNT_RUNTIME_ARG_NAME => amount,
    };

    runtime::call_contract::<()>(token_contract, TRANSFER_ENTRY_POINT_NAME, transfer_args);
}

#[no_mangle]
extern "C" fn transfer_from_as_stored_contract() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    let transfer_from_args = runtime_args! {
        OWNER_RUNTIME_ARG_NAME => owner,
        RECIPIENT_RUNTIME_ARG_NAME => recipient,
        AMOUNT_RUNTIME_ARG_NAME => amount,
    };

    runtime::call_contract::<()>(
        token_contract,
        TRANSFER_FROM_ENTRY_POINT_NAME,
        transfer_from_args,
    );
}

#[no_mangle]
extern "C" fn approve_as_stored_contract() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    let approve_args = runtime_args! {
        SPENDER_RUNTIME_ARG_NAME => spender,
        AMOUNT_RUNTIME_ARG_NAME => amount,
    };

    runtime::call_contract::<()>(token_contract, APPROVE_ENTRY_POINT_NAME, approve_args);
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    let check_total_supply_entrypoint = EntryPoint::new(
        String::from(CHECK_TOTAL_SUPPLY_ENTRY_POINT_NAME),
        vec![Parameter::new(
            TOKEN_CONTRACT_RUNTIME_ARG_NAME,
            ContractHash::cl_type(),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_balance_of_entrypoint = EntryPoint::new(
        String::from(CHECK_BALANCE_OF_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(ADDRESS_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_allowance_of_entrypoint = EntryPoint::new(
        String::from(CHECK_ALLOWANCE_OF_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let transfer_as_stored_contract_entrypoint = EntryPoint::new(
        String::from(TRANSFER_AS_STORED_CONTRACT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let approve_as_stored_contract_entrypoint = EntryPoint::new(
        String::from(APPROVE_AS_STORED_CONTRACT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let transfer_from_as_stored_contract_entrypoint = EntryPoint::new(
        String::from(TRANSFER_FROM_AS_STORED_CONTRACT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(check_total_supply_entrypoint);
    entry_points.add_entry_point(check_balance_of_entrypoint);
    entry_points.add_entry_point(check_allowance_of_entrypoint);
    entry_points.add_entry_point(transfer_as_stored_contract_entrypoint);
    entry_points.add_entry_point(approve_as_stored_contract_entrypoint);
    entry_points.add_entry_point(transfer_from_as_stored_contract_entrypoint);

    let (_contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(ERC20_TEST_CALL_KEY.to_string()),
        None,
    );
}
