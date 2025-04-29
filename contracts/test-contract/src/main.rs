#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
};
use casper_contract::{
    self,
    contract_api::{
        runtime::{self, put_key},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, AddressableEntityHash, ApiError, CLTyped,
    EntityEntryPoint as EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter,
    RuntimeArgs, U256,
};
use cep18::constants::{
    ARG_ADDRESS, ARG_AMOUNT, ARG_OWNER, ARG_RECIPIENT, ARG_SPENDER, ENTRY_POINT_ALLOWANCE,
    ENTRY_POINT_APPROVE, ENTRY_POINT_BALANCE_OF, ENTRY_POINT_TOTAL_SUPPLY, ENTRY_POINT_TRANSFER,
    ENTRY_POINT_TRANSFER_FROM,
};
use cep18_test_contract::constants::{
    ARG_TOKEN_CONTRACT, CEP18_TEST_CONTRACT_NAME, CEP18_TEST_CONTRACT_PACKAGE_NAME,
    ENTRY_POINT_APPROVE_AS_STORED_CONTRACT, ENTRY_POINT_CHECK_ALLOWANCE_OF,
    ENTRY_POINT_CHECK_BALANCE_OF, ENTRY_POINT_CHECK_TOTAL_SUPPLY,
    ENTRY_POINT_TRANSFER_AS_STORED_CONTRACT, ENTRY_POINT_TRANSFER_FROM_AS_STORED_CONTRACT,
    RESULT_KEY,
};

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
    let token_contract: AddressableEntityHash = runtime::get_named_arg::<Key>(ARG_TOKEN_CONTRACT)
        .into_entity_hash()
        .unwrap_or_revert_with(ApiError::User(61000));
    let total_supply: U256 = runtime::call_contract(
        token_contract.into(),
        ENTRY_POINT_TOTAL_SUPPLY,
        RuntimeArgs::default(),
    );
    store_result(total_supply);
}

#[no_mangle]
extern "C" fn check_balance_of() {
    let token_contract: AddressableEntityHash = runtime::get_named_arg::<Key>(ARG_TOKEN_CONTRACT)
        .into_entity_hash()
        .unwrap_or_revert_with(ApiError::User(61001));
    let address: Key = runtime::get_named_arg(ARG_ADDRESS);

    let balance_args = runtime_args! {
        ARG_ADDRESS => address,
    };
    let result: U256 =
        runtime::call_contract(token_contract.into(), ENTRY_POINT_BALANCE_OF, balance_args);

    store_result(result);
}

#[no_mangle]
extern "C" fn check_allowance_of() {
    let token_contract: AddressableEntityHash = runtime::get_named_arg::<Key>(ARG_TOKEN_CONTRACT)
        .into_entity_hash()
        .unwrap_or_revert_with(ApiError::User(61002));
    let owner: Key = runtime::get_named_arg(ARG_OWNER);
    let spender: Key = runtime::get_named_arg(ARG_SPENDER);

    let allowance_args = runtime_args! {
        ARG_OWNER => owner,
        ARG_SPENDER => spender,
    };
    let result: U256 =
        runtime::call_contract(token_contract.into(), ENTRY_POINT_ALLOWANCE, allowance_args);

    store_result(result);
}

#[no_mangle]
extern "C" fn transfer_as_stored_contract() {
    let token_contract: AddressableEntityHash = runtime::get_named_arg::<Key>(ARG_TOKEN_CONTRACT)
        .into_entity_hash()
        .unwrap_or_revert_with(ApiError::User(61003));
    let recipient: Key = runtime::get_named_arg(ARG_RECIPIENT);
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);

    let transfer_args = runtime_args! {
        ARG_RECIPIENT => recipient,
        ARG_AMOUNT => amount,
    };

    runtime::call_contract::<()>(token_contract.into(), ENTRY_POINT_TRANSFER, transfer_args);
}

#[no_mangle]
extern "C" fn transfer_from_as_stored_contract() {
    let token_contract: AddressableEntityHash = runtime::get_named_arg::<Key>(ARG_TOKEN_CONTRACT)
        .into_entity_hash()
        .unwrap_or_revert_with(ApiError::User(61004));
    let owner: Key = runtime::get_named_arg(ARG_OWNER);
    let recipient: Key = runtime::get_named_arg(ARG_RECIPIENT);
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);

    let transfer_from_args = runtime_args! {
        ARG_OWNER => owner,
        ARG_RECIPIENT => recipient,
        ARG_AMOUNT => amount,
    };

    runtime::call_contract::<()>(
        token_contract.into(),
        ENTRY_POINT_TRANSFER_FROM,
        transfer_from_args,
    );
}

#[no_mangle]
extern "C" fn approve_as_stored_contract() {
    let token_contract: AddressableEntityHash = runtime::get_named_arg::<Key>(ARG_TOKEN_CONTRACT)
        .into_entity_hash()
        .unwrap_or_revert_with(ApiError::User(61005));
    let spender: Key = runtime::get_named_arg(ARG_SPENDER);
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);

    let approve_args = runtime_args! {
        ARG_SPENDER => spender,
        ARG_AMOUNT => amount,
    };

    runtime::call_contract::<()>(token_contract.into(), ENTRY_POINT_APPROVE, approve_args);
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    let check_total_supply_entrypoint = EntryPoint::new(
        String::from(ENTRY_POINT_CHECK_TOTAL_SUPPLY),
        vec![Parameter::new(
            ARG_TOKEN_CONTRACT,
            AddressableEntityHash::cl_type(),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    );
    let check_balance_of_entrypoint = EntryPoint::new(
        String::from(ENTRY_POINT_CHECK_BALANCE_OF),
        vec![
            Parameter::new(ARG_TOKEN_CONTRACT, AddressableEntityHash::cl_type()),
            Parameter::new(ARG_ADDRESS, Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    );
    let check_allowance_of_entrypoint = EntryPoint::new(
        String::from(ENTRY_POINT_CHECK_ALLOWANCE_OF),
        vec![
            Parameter::new(ARG_TOKEN_CONTRACT, AddressableEntityHash::cl_type()),
            Parameter::new(ARG_OWNER, Key::cl_type()),
            Parameter::new(ARG_SPENDER, Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    );

    let transfer_as_stored_contract_entrypoint = EntryPoint::new(
        String::from(ENTRY_POINT_TRANSFER_AS_STORED_CONTRACT),
        vec![
            Parameter::new(ARG_TOKEN_CONTRACT, AddressableEntityHash::cl_type()),
            Parameter::new(ARG_RECIPIENT, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    );

    let approve_as_stored_contract_entrypoint = EntryPoint::new(
        String::from(ENTRY_POINT_APPROVE_AS_STORED_CONTRACT),
        vec![
            Parameter::new(ARG_TOKEN_CONTRACT, AddressableEntityHash::cl_type()),
            Parameter::new(ARG_SPENDER, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    );

    let transfer_from_as_stored_contract_entrypoint = EntryPoint::new(
        String::from(ENTRY_POINT_TRANSFER_FROM_AS_STORED_CONTRACT),
        vec![
            Parameter::new(ARG_TOKEN_CONTRACT, AddressableEntityHash::cl_type()),
            Parameter::new(ARG_OWNER, Key::cl_type()),
            Parameter::new(ARG_RECIPIENT, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    );

    entry_points.add_entry_point(check_total_supply_entrypoint);
    entry_points.add_entry_point(check_balance_of_entrypoint);
    entry_points.add_entry_point(check_allowance_of_entrypoint);
    entry_points.add_entry_point(transfer_as_stored_contract_entrypoint);
    entry_points.add_entry_point(approve_as_stored_contract_entrypoint);
    entry_points.add_entry_point(transfer_from_as_stored_contract_entrypoint);

    let (contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(CEP18_TEST_CONTRACT_PACKAGE_NAME.to_string()),
        None,
        None,
    );

    put_key(CEP18_TEST_CONTRACT_NAME, Key::from(contract_hash));
}
