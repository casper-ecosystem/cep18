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
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U256,
};

#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = get_key("symbol");
    ret(val)
}

#[no_mangle]
pub extern "C" fn decimals() {
    let val: u8 = get_key("decimals");
    ret(val)
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val: U256 = get_key("total_supply");
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
pub extern "C" fn transfer_from() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transfer_from(owner, recipient, amount);
}

#[no_mangle]
pub extern "C" fn call() {
    let tokenName: String = runtime::get_named_arg("tokenName");
    let tokenSymbol: String = runtime::get_named_arg("tokenSymbol");
    let tokenTotalSupply: U256 = runtime::get_named_arg("tokenTotalSupply");

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("decimals", vec![], CLType::U8));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U32));
    entry_points.add_entry_point(endpoint(
        "transfer",
        vec![
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "allowance",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("spender", AccountHash::cl_type()),
        ],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "approve",
        vec![
            Parameter::new("spender", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_from",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));

    let mut named_keys = NamedKeys::new();
    named_keys.insert("name".to_string(), storage::new_uref(tokenName).into());
    named_keys.insert("symbol".to_string(), storage::new_uref(tokenSymbol).into());
    named_keys.insert("decimals".to_string(), storage::new_uref(18u8).into());
    named_keys.insert(
        "total_supply".to_string(),
        storage::new_uref(tokenTotalSupply).into(),
    );
    named_keys.insert(
        balance_key(&runtime::get_caller()),
        storage::new_uref(tokenTotalSupply).into(),
    );

    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("ERC20", contract_hash.into());
    runtime::put_key("ERC20_hash", storage::new_uref(contract_hash).into());
}

fn _transfer(sender: AccountHash, recipient: AccountHash, amount: U256) {
    let sender_key = balance_key(&sender);
    let recipient_key = balance_key(&recipient);
    let new_sender_balance: U256 = (get_key::<U256>(&sender_key) - amount);
    set_key(&sender_key, new_sender_balance);
    let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key) + amount);
    set_key(&recipient_key, new_recipient_balance);
}

fn _transfer_from(owner: AccountHash, recipient: AccountHash, amount: U256) {
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
    format!("balances_{}", account)
}

fn allowance_key(owner: &AccountHash, sender: &AccountHash) -> String {
    format!("allowances_{}_{}", owner, sender)
}

fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
