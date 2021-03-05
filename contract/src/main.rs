#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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
use core::{
    ops::Deref,
    ops::DerefMut, 
};

#[derive(Default)]
struct CasperVariable<T: FromBytes + ToBytes + CLTyped + Default>(String, Option<T>, bool);
impl<T: FromBytes + ToBytes + CLTyped + Default> Deref for CasperVariable<T>  where T: FromBytes + CLTyped + Default {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}
impl<T: FromBytes + ToBytes + CLTyped + Default> DerefMut for CasperVariable<T>  where T: FromBytes + CLTyped + Default {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.change(true);
        &mut self.1
    }
}
impl<T: FromBytes + ToBytes + CLTyped + Default> CasperVariable<T> {
    fn change(&mut self, state: bool) {
        self.2 = state;
    }
    fn is_chanaged(&self) -> bool {
        self.2
    }
}

#[derive(Default)]
struct CasperMap<T: FromBytes + ToBytes + CLTyped + Default>(String, Option<T>, bool);
impl<T: FromBytes + ToBytes + CLTyped + Default> CasperMap<T> {
    fn change(&mut self, state: bool) {
        self.2 = state;
    }
    fn is_chanaged(&self) -> bool {
        self.2
    }
    fn get(&self, key: String) -> T {
        get_key::<T>(key.as_str())
    }
    fn set(&self, key: String, value: T) {
        set_key(key.as_str(), value)
    }
}

trait GetKey {
    fn get_key(&self) -> String;
}
impl GetKey for AccountHash {
    fn get_key(&self) -> String {
        balance_key(self)
    }
}
impl GetKey for (AccountHash, AccountHash) {
    fn get_key(&self) -> String {
        allowance_key(&self.0, &self.1)
    }
}

struct ERC20Context {
    name: CasperVariable<String>,
    symbol: CasperVariable<String>,
    decimals: CasperVariable<u8>,
    totalSupply: CasperVariable<U256>,
    balances: CasperMap<U256>,
    allowances: CasperMap<U256>
}

impl Default for ERC20Context {
    fn default() -> Self {
        ERC20Context {
            name : CasperVariable::<String>("_name".to_string(), Some(get_key("_name")), false),
            symbol : CasperVariable::<String>("_symbol".to_string(), Some(get_key("_symbol")), false),
            decimals : CasperVariable::<u8>("_decimals".to_string(), Some(get_key("_decimals")), false),
            totalSupply : CasperVariable::<U256>("_totalsupply".to_string(), Some(get_key("_totalsupply")), false),
            balances: CasperMap::<U256>::default(),
            allowances: CasperMap::<U256>::default()
        }
    }
}

impl ERC20Context {
    pub fn new(tokenName: String, tokenSymbol: String, tokenTotalSupply: U256) -> Self {
        ERC20Context {
            name : CasperVariable::<String>("_name".to_string(), Some(tokenName), true),
            symbol : CasperVariable::<String>("_symbol".to_string(), Some(tokenSymbol), true),
            decimals : CasperVariable::<u8>("_decimals".to_string(), Some(18u8), true),
            totalSupply : CasperVariable::<U256>("_totalsupply".to_string(), Some(tokenTotalSupply), true),
            balances: CasperMap::<U256>::default(),
            allowances: CasperMap::<U256>::default()
        }
    }

    pub fn save(&mut self) {
        if self.name.is_chanaged() {
            set_key(self.name.0.as_str(), self.name.1.clone().unwrap());
        }
        if self.symbol.is_chanaged() {
            set_key(self.symbol.0.as_str(), self.symbol.1.clone().unwrap());
        }
        if self.decimals.is_chanaged() {
            set_key(self.decimals.0.as_str(), self.decimals.1.clone().unwrap());
        }
        if self.totalSupply.is_chanaged() {
            set_key(self.totalSupply.0.as_str(), self.totalSupply.1.clone().unwrap());
        }
        if self.balances.is_chanaged() {
            set_key(self.balances.0.as_str(), self.balances.1.clone().unwrap());
        }
        if self.allowances.is_chanaged() {
            set_key(self.allowances.0.as_str(), self.allowances.1.clone().unwrap());
        }
    }
}

#[casperlabs_contract]
mod ERC20 {
    use crate::{ERC20Context, get_key};


    #[casperlabs_constructor]
    fn constructor(tokenName: String, tokenSymbol: String, tokenTotalSupply: U256) {
        let mut context = ERC20Context::default();
        let balanceKey = runtime::get_caller().get_key();
        *context.name = Some(tokenName);
        *context.symbol = Some(tokenSymbol);
        *context.decimals = Some(18u8);
        *context.totalSupply = Some(tokenTotalSupply);
        context.balances.set(balanceKey, tokenTotalSupply);
        context.save();
    }

    #[casperlabs_method]
    fn name() -> String {
        let context = ERC20Context::default();
        context.name.clone().unwrap()
    }

    #[casperlabs_method]
    fn symbol() -> String {
        let context = ERC20Context::default();
        context.symbol.clone().unwrap()
    }

    #[casperlabs_method]
    fn decimals() -> u8 {
        let context = ERC20Context::default();
        context.decimals.clone().unwrap()
    }

    #[casperlabs_method]
    fn totalSupply() -> U256 {
        let context = ERC20Context::default();
        context.totalSupply.clone().unwrap()
    }

    #[casperlabs_method]
    fn transfer(recipient: AccountHash, amount: U256) {
        let mut context = ERC20Context::default();
        let owner = runtime::get_caller();

        let owner_balance = context.balances.get(owner.get_key());
        let recipient_balance = context.balances.get(recipient.get_key());

        if (owner_balance < amount)  {
            runtime::revert(1);
        }

        context.balances.set(owner.get_key(), owner_balance - amount);
        context.balances.set(recipient.get_key(), recipient_balance + amount);

        context.save();
    }

    #[casperlabs_method]
    fn balance_of(account: AccountHash) -> U256 {
        let context = ERC20Context::default();
        context.balances.get(account.get_key())
    }

    #[casperlabs_method]
    fn allowance(owner: AccountHash, spender: AccountHash) -> U256 {
        let context = ERC20Context::default();
        context.allowances.get((owner, spender).get_key())
    }

    #[casperlabs_method]
    fn approve(spender: AccountHash, amount: U256) {
        let mut context = ERC20Context::default();
        let owner = runtime::get_caller();

        context.allowances.set((owner, spender).get_key(), amount);

        context.save();
    }

    #[casperlabs_method]
    fn transferFrom(owner: AccountHash, recipient: AccountHash, amount: U256) {
        let mut context = ERC20Context::default();

        let owner_balance = context.balances.get(owner.get_key());
        let recipient_balance = context.balances.get(recipient.get_key());

        if (owner_balance < amount)  {
            runtime::revert(1);
        }

        context.balances.set(owner.get_key(), owner_balance - amount);
        context.balances.set(recipient.get_key(), recipient_balance + amount);
        
        let spender = runtime::get_caller();
        let owner_spender_allowance = context.allowances.get((owner, spender).get_key());
        context.allowances.set((owner, spender).get_key(), owner_spender_allowance - amount);
        
        context.save();
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
