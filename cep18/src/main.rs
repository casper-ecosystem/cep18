#![no_std]
#![no_main]

extern crate alloc;

mod allowances;
mod balances;
pub mod constants;
pub mod entry_points;
mod error;
mod events;
mod modalities;
mod utils;

use alloc::{
    format,
    string::{String, ToString},
};

use allowances::{get_allowances_uref, read_allowance_from, write_allowance_to};
use balances::{get_balances_uref, read_balance_from, transfer_balance, write_balance_to};
use entry_points::generate_entry_points;

use casper_contract::{
    contract_api::{
        runtime::{self, get_caller, get_named_arg, put_key},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, runtime_args, CLValue, Key, RuntimeArgs, U256};

use constants::{
    ACCESS_KEY_NAME_PREFIX, ADDRESS, ALLOWANCES, AMOUNT, BALANCES, CONTRACT_NAME_PREFIX,
    CONTRACT_VERSION_PREFIX, DECIMALS, ENTRY_POINT_INIT, EVENTS_MODE, HASH_KEY_NAME_PREFIX, NAME,
    OWNER, PACKAGE_HASH, RECIPIENT, SPENDER, SYMBOL, TOTAL_SUPPLY,
};
pub use error::Cep18Error;
use events::{
    Burn, DecreaseAllowance, Event, IncreaseAllowance, Mint, SetAllowance, Transfer, TransferFrom,
};
use utils::{get_total_supply_uref, read_total_supply_from, write_total_supply_to};

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
    let address: Key = runtime::get_named_arg(ADDRESS);
    let balances_uref = get_balances_uref();
    let balance = balances::read_balance_from(balances_uref, address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Key = runtime::get_named_arg(OWNER);
    let spender: Key = runtime::get_named_arg(SPENDER);
    let allowances_uref = get_allowances_uref();
    let val: U256 = read_allowance_from(allowances_uref, owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Key = runtime::get_named_arg(SPENDER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let allowances_uref = get_allowances_uref();
    write_allowance_to(allowances_uref, owner, spender, amount);
    events::record_event_dictionary(Event::SetAllowance(SetAllowance {
        owner,
        spender,
        allowance: amount,
    }))
}

#[no_mangle]
pub extern "C" fn decrease_allowance() {
    let spender: Key = runtime::get_named_arg(SPENDER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let allowances_uref = get_allowances_uref();
    let current_allowance = read_allowance_from(allowances_uref, owner, spender);
    let new_allowance = current_allowance.saturating_sub(amount);
    write_allowance_to(allowances_uref, owner, spender, new_allowance);
    events::record_event_dictionary(Event::DecreaseAllowance(DecreaseAllowance {
        owner,
        spender,
        decr_by: amount,
        allowance: new_allowance,
    }))
}

#[no_mangle]
pub extern "C" fn increase_allowance() {
    let spender: Key = runtime::get_named_arg(SPENDER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let allowances_uref = get_allowances_uref();
    let current_allowance = read_allowance_from(allowances_uref, owner, spender);
    let new_allowance = current_allowance.saturating_add(amount);
    write_allowance_to(allowances_uref, owner, spender, new_allowance);
    events::record_event_dictionary(Event::IncreaseAllowance(IncreaseAllowance {
        owner,
        spender,
        allowance: new_allowance,
        inc_by: amount,
    }))
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Key = runtime::get_named_arg(RECIPIENT);
    let amount: U256 = runtime::get_named_arg(AMOUNT);

    let sender = utils::get_immediate_caller_address().unwrap_or_revert();

    transfer_balance(sender, recipient, amount).unwrap_or_revert();
    events::record_event_dictionary(Event::Transfer(Transfer {
        sender,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Key = runtime::get_named_arg(OWNER);
    let recipient: Key = runtime::get_named_arg(RECIPIENT);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let spender = utils::get_immediate_caller_address().unwrap_or_revert();
    if amount.is_zero() {
        return;
    }

    let allowances_uref = get_allowances_uref();
    let spender_allowance: U256 = read_allowance_from(allowances_uref, owner, spender);
    let new_spender_allowance = spender_allowance
        .checked_sub(amount)
        .ok_or(Cep18Error::InsufficientAllowance)
        .unwrap_or_revert();

    transfer_balance(owner, recipient, amount).unwrap_or_revert();
    write_allowance_to(allowances_uref, owner, spender, new_spender_allowance);
    events::record_event_dictionary(Event::TransferFrom(TransferFrom {
        spender,
        owner,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn mint() {
    let owner: Key = runtime::get_named_arg(OWNER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);

    let balances_uref = get_balances_uref();
    let total_supply_uref = get_total_supply_uref();
    let new_balance = {
        let balance = read_balance_from(balances_uref, owner);
        balance
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    let new_total_supply = {
        let total_supply: U256 = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    write_balance_to(balances_uref, owner, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
    events::record_event_dictionary(Event::Mint(Mint {
        recipient: owner,
        amount,
    }))
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
            .ok_or(Cep18Error::InsufficientBalance)
            .unwrap_or_revert()
    };
    let new_total_supply = {
        let total_supply = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_sub(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    write_balance_to(balances_uref, owner, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
    events::record_event_dictionary(Event::Burn(Burn { owner, amount }))
}

#[no_mangle]
pub extern "C" fn init() {
    let package_hash = get_named_arg::<Key>(PACKAGE_HASH);
    put_key(PACKAGE_HASH, package_hash);
    storage::new_dictionary(ALLOWANCES).unwrap_or_revert();
    let balances_uref = storage::new_dictionary(BALANCES).unwrap_or_revert();
    let initial_supply = runtime::get_named_arg(TOTAL_SUPPLY);
    let caller = get_caller();
    write_balance_to(balances_uref, caller.into(), initial_supply);
}

#[no_mangle]
pub extern "C" fn migrate() {}

pub fn install_contract() {
    let name: String = runtime::get_named_arg(NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL);
    let decimals: u8 = runtime::get_named_arg(DECIMALS);
    let total_supply: U256 = runtime::get_named_arg(TOTAL_SUPPLY);
    let events_mode: u8 =
        utils::get_optional_named_arg_with_user_errors(EVENTS_MODE, Cep18Error::InvalidEventsMode)
            .unwrap_or(0u8);
    let mut named_keys = NamedKeys::new();
    named_keys.insert(NAME.to_string(), storage::new_uref(name.clone()).into());
    named_keys.insert(SYMBOL.to_string(), storage::new_uref(symbol).into());
    named_keys.insert(DECIMALS.to_string(), storage::new_uref(decimals).into());
    named_keys.insert(
        TOTAL_SUPPLY.to_string(),
        storage::new_uref(total_supply).into(),
    );
    named_keys.insert(
        EVENTS_MODE.to_string(),
        storage::new_uref(events_mode).into(),
    );

    let entry_points = generate_entry_points();

    let hash_key_name = format!("{HASH_KEY_NAME_PREFIX}{name}");

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(hash_key_name.clone()),
        Some(format!("{ACCESS_KEY_NAME_PREFIX}{name}")),
    );
    let package_hash = runtime::get_key(&hash_key_name).unwrap_or_revert();

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
        runtime_args! {TOTAL_SUPPLY => total_supply, PACKAGE_HASH => package_hash},
    );
}

#[no_mangle]
pub extern "C" fn call() {
    install_contract()
}
