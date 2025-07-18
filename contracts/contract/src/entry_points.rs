//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};
use casper_types::{
    CLType, CLTyped, EntityEntryPoint as EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Key, Parameter, U256,
};

use crate::constants::{
    ARG_ADDRESS, ARG_AMOUNT, ARG_EVENTS_MODE, ARG_OWNER, ARG_RECIPIENT, ARG_SPENDER,
    ENTRY_POINT_ALLOWANCE, ENTRY_POINT_APPROVE, ENTRY_POINT_BALANCE_OF, ENTRY_POINT_BURN,
    ENTRY_POINT_CHANGE_EVENTS_MODE, ENTRY_POINT_CHANGE_SECURITY, ENTRY_POINT_DECIMALS,
    ENTRY_POINT_DECREASE_ALLOWANCE, ENTRY_POINT_INCREASE_ALLOWANCE, ENTRY_POINT_INIT,
    ENTRY_POINT_MINT, ENTRY_POINT_NAME, ENTRY_POINT_SYMBOL, ENTRY_POINT_TOTAL_SUPPLY,
    ENTRY_POINT_TRANSFER, ENTRY_POINT_TRANSFER_FROM,
};

/// Returns the `name` entry point.
pub fn name() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `symbol` entry point.
pub fn symbol() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_SYMBOL),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `transfer_from` entry point.
pub fn transfer_from() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_TRANSFER_FROM),
        vec![
            Parameter::new(ARG_OWNER, Key::cl_type()),
            Parameter::new(ARG_RECIPIENT, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `allowance` entry point.
pub fn allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_ALLOWANCE),
        vec![
            Parameter::new(ARG_OWNER, Key::cl_type()),
            Parameter::new(ARG_SPENDER, Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `approve` entry point.
pub fn approve() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_APPROVE),
        vec![
            Parameter::new(ARG_SPENDER, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `increase_allowance` entry point.
pub fn increase_allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_INCREASE_ALLOWANCE),
        vec![
            Parameter::new(ARG_SPENDER, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `decrease_allowance` entry point.
pub fn decrease_allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_DECREASE_ALLOWANCE),
        vec![
            Parameter::new(ARG_SPENDER, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `transfer` entry point.
pub fn transfer() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_TRANSFER),
        vec![
            Parameter::new(ARG_RECIPIENT, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_BALANCE_OF),
        vec![Parameter::new(ARG_ADDRESS, Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `total_supply` entry point.
pub fn total_supply() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_TOTAL_SUPPLY),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `decimals` entry point.
pub fn decimals() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_DECIMALS),
        Vec::new(),
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `burn` entry point.
pub fn burn() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_BURN),
        vec![
            Parameter::new(ARG_OWNER, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `mint` entry point.
pub fn mint() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_MINT),
        vec![
            Parameter::new(ARG_OWNER, Key::cl_type()),
            Parameter::new(ARG_AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `change_events_mode` entry point.
pub fn change_events_mode() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_CHANGE_EVENTS_MODE),
        vec![Parameter::new(ARG_EVENTS_MODE, u8::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `change_security` entry point.
pub fn change_security() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_CHANGE_SECURITY),
        vec![
            // Optional Arguments (can be added or omitted when calling):
            /*
            - "admin_list" : Vec<Key>
            - "minter_list" : Vec<Key>
            - "none_list" : Vec<Key>
            */
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the `init` entry point.
pub fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_INIT),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the default set of CEP-18 token entry points.
pub fn generate_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(name());
    entry_points.add_entry_point(symbol());
    entry_points.add_entry_point(decimals());
    entry_points.add_entry_point(total_supply());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(transfer());
    entry_points.add_entry_point(approve());
    entry_points.add_entry_point(allowance());
    entry_points.add_entry_point(decrease_allowance());
    entry_points.add_entry_point(increase_allowance());
    entry_points.add_entry_point(transfer_from());
    entry_points.add_entry_point(change_security());
    entry_points.add_entry_point(burn());
    entry_points.add_entry_point(mint());
    entry_points.add_entry_point(change_events_mode());
    entry_points
}
