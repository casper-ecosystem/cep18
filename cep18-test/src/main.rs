#![no_std]
#![no_main]

extern crate alloc;

pub mod balances;
pub mod error;

use alloc::{
    string::{String, ToString},
    format,
    vec,
    vec::Vec
};
use balances::{get_balances_uref, read_balance_from, write_balance_to};
use error::Error;
use core::{ops::{Deref, DerefMut}, convert::TryInto};

use casper_contract::{contract_api::{runtime::{self, call_versioned_contract, call_contract, get_caller}, storage}, unwrap_or_revert::UnwrapOrRevert};

use casper_types::{
    account::AccountHash, CLType, CLTyped, CLValue, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256, RuntimeArgs, runtime_args, Key, contracts::NamedKeys, URef, ApiError
};

pub const MINT: &str = "mint";
pub const BURN: &str = "burn";
pub const NAME: &str = "name";
pub const SYMBOL: &str = "symbol";
pub const DECIMALS: &str = "decimals";
pub const BALANCES: &str = "balances";
pub const ALLOWANCES: &str = "allowances";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const BALANCE_OF: &str = "balance_of";
pub const INIT: &str = "init";

pub const ADDRESS: &str = "address";
pub const OWNER: &str = "owner";
pub const AMOUNT: &str = "amount";
pub const HASH_KEY_NAME_PREFIX: &str = "cep18_contract_package";
pub const ACCESS_KEY_NAME_PREFIX: &str = "cep18_contract_package_access_";
pub const CONTRACT_NAME_PREFIX: &str = "cep18_contract_hash_";
pub const CONTRACT_VERSION_PREFIX: &str = "cep18_contract_version_";
const TEST_CONTRACT_KEY_NAME: &str = "test_contract";
const TOKEN_NAME: &str = "CasperTest";
const TOKEN_SYMBOL: &str = "CSPRT";
const TOKEN_DECIMALS: u8 = 8;
const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;

const TOKEN_OWNER_ADDRESS_1: Key = Key::Account(AccountHash::new([42; 32]));
const TOKEN_OWNER_AMOUNT_1: u64 = 1_000_000;
const TOKEN_OWNER_ADDRESS_2: Key = Key::Hash([42; 32]);
const TOKEN_OWNER_AMOUNT_2: u64 = 2_000_000;

/// Gets [`URef`] under a name.
pub(crate) fn get_uref(name: &str) -> URef {
    let key = runtime::get_key(name)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert();
    key.try_into().unwrap_or_revert()
}


pub fn get_total_supply_uref() -> URef {
    get_uref(TOTAL_SUPPLY)
}

pub(crate) fn read_total_supply_from(uref: URef) -> U256 {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_total_supply_to(uref: URef, value: U256) {
    storage::write(uref, value);
}

#[no_mangle]
pub extern "C" fn total_supply() {
    runtime::ret(CLValue::from_t(read_total_supply_from(get_total_supply_uref())).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Key = runtime::get_named_arg(ADDRESS);
    let balances_uref = get_balances_uref(); 
    let balance = balances::read_balance_from(balances_uref, address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn mint(){
    let owner: Key = runtime::get_named_arg(OWNER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);

    let balances_uref = get_balances_uref(); 
    let total_supply_uref = get_total_supply_uref(); 
    let new_balance = {
        let balance = read_balance_from(balances_uref, owner);
        balance.checked_add(amount).ok_or(Error::Overflow).unwrap_or_revert()
    };
    let new_total_supply = {
        let total_supply: U256 = read_total_supply_from(total_supply_uref);
        total_supply.checked_add(amount).ok_or(Error::Overflow).unwrap_or_revert()
    };
    write_balance_to(balances_uref, owner, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
}

#[no_mangle]
pub extern "C" fn burn() {
    let owner: Key = runtime::get_named_arg(OWNER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let balances_uref = get_balances_uref(); 
    let total_supply_uref = get_total_supply_uref(); 
    let new_balance = {
        let balance = read_balance_from(balances_uref, owner);
        balance
            .checked_sub(amount)
            .ok_or(Error::InsufficientBalance).unwrap_or_revert()
    };
    let new_total_supply = {
        let total_supply = read_total_supply_from(total_supply_uref);
        total_supply.checked_sub(amount).ok_or(Error::Overflow).unwrap_or_revert()
    };
    write_balance_to(balances_uref, owner, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
}

#[no_mangle]
pub extern "C" fn init() {
    let balances_uref = storage::new_dictionary(BALANCES).unwrap_or_revert();
    let allowances_uref = storage::new_dictionary(ALLOWANCES).unwrap_or_revert();
    let initial_supply = runtime::get_named_arg(TOTAL_SUPPLY);
    let caller = get_caller();
    write_balance_to(balances_uref, caller.into(), initial_supply);
}

//TEST_CONTRACT_KEY_NAME
#[no_mangle]
fn call() {
    let name : String = runtime::get_named_arg(NAME);
    let symbol : String = runtime::get_named_arg(SYMBOL);
    let decimals : u8 = runtime::get_named_arg(DECIMALS);
    let total_supply : U256 = runtime::get_named_arg(TOTAL_SUPPLY);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(NAME.to_string(), storage::new_uref(name.clone()).into());
    named_keys.insert(SYMBOL.to_string(), storage::new_uref(symbol).into());
    named_keys.insert(DECIMALS.to_string(), storage::new_uref(decimals).into());
    named_keys.insert(TOTAL_SUPPLY.to_string(), storage::new_uref(total_supply).into());

    let mut entry_points = EntryPoints::new();

    let balance_of_entrypoint =
    EntryPoint::new(
        String::from(BALANCE_OF),
        vec![Parameter::new(ADDRESS, Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let total_supply_entrypoint  =
    EntryPoint::new(
        String::from(TOTAL_SUPPLY),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    
    let mint_entrypoint = EntryPoint::new(
        MINT,
        vec![
            Parameter::new(OWNER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        // NOTE: `Public` access rights allows any user and context to call this entrypoint.
        // If security is required we suggest developers to implement additional security code
        // inside public entrypoints.
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let burn_entrypoint = EntryPoint::new(
        BURN,
        vec![
            Parameter::new(OWNER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        // NOTE: `Public` access rights allows any user and context to call this entrypoint.
        // If security is required we suggest developers to implement additional security code
        // inside public entrypoints.
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let init_entrypoint = EntryPoint::new(
        INIT,
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(balance_of_entrypoint);
    entry_points.add_entry_point(total_supply_entrypoint);
    entry_points.add_entry_point(mint_entrypoint);
    entry_points.add_entry_point(burn_entrypoint);
    entry_points.add_entry_point(init_entrypoint);

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("test_contract_package_hash".to_string()),
        Some("test_contract_access_key".to_string()),
    );

    // Store contract_hash and contract_version under the keys CONTRACT_NAME and CONTRACT_VERSION
    runtime::put_key(
        "test_contract_hash",
        contract_hash.into(),
    );
    runtime::put_key(
        "test_contract_version",
        storage::new_uref(contract_version).into(),
    );

    // Call contract to initialize it
    runtime::call_contract::<()>(
        contract_hash,
        INIT,
        runtime_args! {TOTAL_SUPPLY => total_supply},
    );

    call_contract::<()>(contract_hash, MINT, runtime_args!{OWNER => TOKEN_OWNER_ADDRESS_1, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1)});
    call_contract::<()>(contract_hash, MINT, runtime_args!{OWNER => TOKEN_OWNER_ADDRESS_2, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_2)});
}
