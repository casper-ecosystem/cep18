#![no_std]
#![no_main]

extern crate alloc;

mod allowances;
mod balances;
pub mod constants;
mod utils;
pub mod entry_points;
mod error;

use alloc::{string::{String, ToString}, format};

use allowances::{write_allowance_to, make_dictionary_item_key, get_allowances_uref, read_allowance_from};
use balances::{get_balances_uref, transfer_balance, read_balance_from, write_balance_to};
use entry_points::generate_entry_points;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, U256, runtime_args, RuntimeArgs, CLValue, Key};

use constants::{
    ALLOWANCES, BALANCES, DECIMALS, HASH_KEY_NAME_PREFIX, ACCESS_KEY_NAME_PREFIX,CONTRACT_NAME_PREFIX, CONTRACT_VERSION_PREFIX,
    NAME, SYMBOL, TOTAL_SUPPLY, AMOUNT_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME, ADDRESS_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, SPENDER_RUNTIME_ARG_NAME, ENTRY_POINT_INIT,
};
pub use error::Error;
use utils::{read_total_supply_from, get_total_supply_uref, write_total_supply_to};

#[no_mangle]
pub extern "C" fn name() {
    runtime::ret(CLValue::from_t(utils::read_from::<String>(NAME)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    runtime::ret(CLValue::from_t(utils::read_from::<String>(SYMBOL)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    runtime::ret(CLValue::from_t(utils::read_from::<u8>(DECIMALS)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    runtime::ret(CLValue::from_t(utils::read_from::<U256>(TOTAL_SUPPLY)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Key = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balances_uref = get_balances_uref(); 
    let balance = balances::read_balance_from(balances_uref, address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Key = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    let sender = utils::get_immediate_caller_address().unwrap_or_revert();

    transfer_balance(sender, recipient, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Key = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let allowances_uref = get_allowances_uref();
    let dictionary_item_key = make_dictionary_item_key(owner, spender);
    storage::dictionary_put(allowances_uref, &dictionary_item_key, amount);
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Key = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Key = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let allowances_uref = get_allowances_uref();
    let val : U256 = read_allowance_from(allowances_uref, owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Key = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Key = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let spender = utils::get_immediate_caller_address().unwrap_or_revert();
    if amount.is_zero() {
        return;
    }

    let allowances_uref = get_allowances_uref();
    let spender_allowance : U256 = read_allowance_from(allowances_uref, owner, spender);
    let new_spender_allowance = spender_allowance
        .checked_sub(amount)
        .ok_or(Error::InsufficientAllowance).unwrap_or_revert();

    transfer_balance(owner, recipient, amount).unwrap_or_revert();
    write_allowance_to(owner, spender, new_spender_allowance);
}

#[no_mangle]
pub extern "C" fn mint(){
    let owner: Key = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

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
    let owner: Key = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
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
    storage::new_dictionary(BALANCES).unwrap_or_revert();
    storage::new_dictionary(ALLOWANCES).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn migrate() {}

pub fn install_contract(){
    let name : String = runtime::get_named_arg(NAME);
    let symbol : String = runtime::get_named_arg(SYMBOL);
    let decimals : u8 = runtime::get_named_arg(DECIMALS);
    let total_supply : U256 = runtime::get_named_arg(TOTAL_SUPPLY);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(NAME.to_string(), storage::new_uref(name.clone()).into());
    named_keys.insert(SYMBOL.to_string(), storage::new_uref(symbol).into());
    named_keys.insert(DECIMALS.to_string(), storage::new_uref(decimals).into());
    named_keys.insert(TOTAL_SUPPLY.to_string(), storage::new_uref(total_supply).into());
    
    let entry_points = generate_entry_points();

    let hash_key_name = format!("{HASH_KEY_NAME_PREFIX}_{name}");

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(hash_key_name),
        Some(format!("{ACCESS_KEY_NAME_PREFIX}{name}")),
    );

    // Store contract_hash and contract_version under the keys CONTRACT_NAME and CONTRACT_VERSION
    runtime::put_key(
        &format!("{CONTRACT_NAME_PREFIX}{name}"),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{CONTRACT_VERSION_PREFIX}{name}"),
        storage::new_uref(contract_version).into(),
    );

    // Call contract to initialize it
    runtime::call_contract::<()>(
        contract_hash,
        ENTRY_POINT_INIT,
        runtime_args! {},
    );
}

#[no_mangle]
pub extern "C" fn call() {
    install_contract()
}