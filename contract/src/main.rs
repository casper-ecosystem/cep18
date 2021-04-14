#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U256,
};

#[no_mangle]
pub extern "C" fn constructor() {
    let tokenName: String = runtime::get_named_arg("tokenName");
    let tokenSymbol: String = runtime::get_named_arg("tokenSymbol");
    let tokenTotalSupply: U256 = runtime::get_named_arg("tokenTotalSupply");
    set_key("_name", tokenName);
    set_key("_symbol", tokenSymbol);
    set_key("_decimals", 18u8);
    let balance = balance_key(&runtime::get_caller());
    set_key(&balance, tokenTotalSupply);
    set_key("_totalSupply", tokenTotalSupply);
}

#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("_name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = get_key("_symbol");
    ret(val)
}

#[no_mangle]
pub extern "C" fn decimals() {
    let val: u8 = get_key("_decimals");
    ret(val)
}

#[no_mangle]
pub extern "C" fn totalSupply() {
    let val: U256 = get_key("_totalSupply");
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: AccountHash = runtime::get_named_arg("account");
    let val: U256 = get_key(&balance_key(&account));
    ret(val)
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let spender: AccountHash = runtime::get_named_arg("spender");
    let val: U256 = get_key(&allowance_key(&owner, &spender));
    ret(val)
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: AccountHash = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    _approve(runtime::get_caller(), spender, amount);
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transfer(runtime::get_caller(), recipient, amount);
}

#[no_mangle]
pub extern "C" fn transferFrom() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transferFrom(owner, recipient, amount);
}

#[no_mangle]
pub extern "C" fn call() {
    let tokenName: String = runtime::get_named_arg("tokenName");
    let tokenSymbol: String = runtime::get_named_arg("tokenSymbol");
    let tokenTotalSupply: U256 = runtime::get_named_arg("tokenTotalSupply");
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let _constructor_access_uref: URef = storage::create_contract_user_group(
        contract_package_hash,
        "constructor_group",
        1,
        BTreeSet::new(),
    )
    .unwrap_or_revert()
    .pop()
    .unwrap_or_revert();
    let constructor_group = Group::new("constructor_group");
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        String::from("constructor"),
        vec![
            Parameter::new("tokenName", CLType::String),
            Parameter::new("tokenSymbol", CLType::String),
            Parameter::new("tokenTotalSupply", CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Groups(vec![constructor_group]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("name"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("symbol"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("decimals"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("totalSupply"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("transfer"),
        vec![
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("balance_of"),
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("allowance"),
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("spender", AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("approve"),
        vec![
            Parameter::new("spender", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("transferFrom"),
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, Default::default());
    runtime::put_key("ERC20", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("ERC20_hash", contract_hash_pack.into());
    runtime::call_contract::<()>(contract_hash, "constructor", {
        let mut named_args = RuntimeArgs::new();
        named_args.insert("tokenName", tokenName).unwrap();
        named_args.insert("tokenSymbol", tokenSymbol).unwrap();
        named_args
            .insert("tokenTotalSupply", tokenTotalSupply)
            .unwrap();
        named_args
    });
}

fn _transfer(sender: AccountHash, recipient: AccountHash, amount: U256) {
    let sender_key = balance_key(&sender);
    let recipient_key = balance_key(&recipient);
    let new_sender_balance: U256 = (get_key::<U256>(&sender_key) - amount);
    set_key(&sender_key, new_sender_balance);
    let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key) + amount);
    set_key(&recipient_key, new_recipient_balance);
}

fn _transferFrom(owner: AccountHash, recipient: AccountHash, amount: U256) {
    let key = allowance_key(&owner, &runtime::get_caller());
    _transfer(owner, recipient, amount);
    _approve(
        owner,
        runtime::get_caller(),
        (get_key::<U256>(&key) - amount),
    );
}

fn _approve(owner: AccountHash, spender: AccountHash, amount: U256) {
    set_key(&allowance_key(&owner, &spender), amount);
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

fn balance_key(account: &AccountHash) -> String {
    format!("_balances_{}", account)
}

fn allowance_key(owner: &AccountHash, sender: &AccountHash) -> String {
    format!("_allowances_{}_{}", owner, sender)
}
