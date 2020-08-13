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

use casperlabs_contract_macro::{casperlabs_constructor, casperlabs_contract, casperlabs_method};
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

#[casperlabs_contract]
mod ERC20 {

    #[casperlabs_constructor]
    fn constructor(tokenName: String, tokenSymbol: String, tokenTotalSupply: U256) {
        set_key("_name", tokenName);
        set_key("_symbol", tokenSymbol);
        set_key("_decimals", 18u8);
        let balance = balance_key(&runtime::get_caller());
        set_key(&balance, tokenTotalSupply);
        set_key("_totalSupply", tokenTotalSupply);
    }

    #[casperlabs_method]
    fn name() -> String {
        get_key("_name")
    }

    #[casperlabs_method]
    fn symbol() -> String {
        get_key("_symbol")
    }

    #[casperlabs_method]
    fn decimals() -> u8 {
        get_key("_decimals")
    }

    #[casperlabs_method]
    fn totalSupply() -> U256 {
        get_key("_totalSupply")
    }

    #[casperlabs_method]
    fn transfer(recipient: AccountHash, amount: U256) {
        _transfer(runtime::get_caller(), recipient, amount);
    }

    #[casperlabs_method]
    fn balance_of(account: AccountHash) -> U256 {
        get_key(&balance_key(&account))
    }

    #[casperlabs_method]
    fn allowance(owner: AccountHash, spender: AccountHash) -> U256 {
        get_key(&allowance_key(&owner, &spender))
    }

    #[casperlabs_method]
    fn approve(spender: AccountHash, amount: U256) {
        _approve(runtime::get_caller(), spender, amount);
    }

    #[casperlabs_method]
    fn transferFrom(owner: AccountHash, recipient: AccountHash, amount: U256) {
        let key = allowance_key(&owner, &runtime::get_caller());
        _transfer(owner, recipient, amount);
        _approve(
            owner,
            runtime::get_caller(),
            (get_key::<U256>(&key) - amount),
        );
    }

    fn _transfer(sender: AccountHash, recipient: AccountHash, amount: U256) {
        let sender_key = balance_key(&sender);
        let recipient_key = balance_key(&recipient);
        let new_sender_balance: U256 = (get_key::<U256>(&sender_key) - amount);
        set_key(&sender_key, new_sender_balance);
        let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key) + amount);
        set_key(&recipient_key, new_recipient_balance);
    }

    fn _approve(owner: AccountHash, spender: AccountHash, amount: U256) {
        set_key(&allowance_key(&owner, &spender), amount);
    }
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
