use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key,
    Parameter, U256,
};

use crate::{init_erc20, ERC20_INSTANCE};

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    unsafe {
        init_erc20();
    };
    ERC20_INSTANCE
        .get()
        .unwrap_or_revert()
        .transfer(recipient, amount);
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimals: u8 = runtime::get_named_arg("decimals");
    let initial_supply: U256 = runtime::get_named_arg("initial_supply");
    unsafe {
        init_erc20();
    };
    ERC20_INSTANCE
        .get()
        .unwrap_or_revert()
        .constructor(name, symbol, decimals, initial_supply);
}

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    unsafe {
        init_erc20();
    };
    ERC20_INSTANCE
        .get()
        .unwrap_or_revert()
        .transfer_from(owner, recipient, amount);
}

#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    unsafe {
        init_erc20();
    };
    ERC20_INSTANCE
        .get()
        .unwrap_or_revert()
        .approve(spender, amount);
}

#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    unsafe {
        init_erc20();
    };
    let ret: U256 = ERC20_INSTANCE.get().unwrap_or_revert().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let ret: U256 = ERC20_INSTANCE
        .get()
        .unwrap_or_revert()
        .allowance(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply() {
    let ret: U256 = ERC20_INSTANCE.get().unwrap_or_revert().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

pub fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("decimals", u8::cl_type()),
            Parameter::new("initial_supply", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
