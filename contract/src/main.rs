#![no_main]
use std::collections::BTreeSet;
use types::{CLTyped, RuntimeArgs};

mod dsl;
use contract_vars::{ERC20Context, Save};
use dsl::{
    runtime, storage,
    types::{account::AccountHash, U256},
    GetKey, Map, UnwrapOrRevert, Variable,
};

#[derive(ERC20Context)]
struct ERC20 {
    token_name: Variable<String>,
    token_symbol: Variable<String>,
    total_supply: Variable<U256>,
    balances: Map<AccountHash, U256>,
    allowances: Map<(AccountHash, AccountHash), U256>,
}

// #[casper_contract]
impl ERC20 {
    pub fn new(token_name: String, token_symbol: String, total_supply: U256) -> ERC20 {
        let mut erc20 = ERC20::default();
        erc20.balances.set(&runtime::get_caller(), total_supply);
        erc20.total_supply = Variable::new(String::from("total_supply"), total_supply);
        erc20.token_name = Variable::new(String::from("token_name"), token_name);
        erc20.token_symbol = Variable::new(String::from("token_symbol"), token_symbol);
        erc20
    }

    pub fn name(&self) -> String {
        self.token_name.get()
    }

    pub fn symbol(&self) -> String {
        self.token_symbol.get()
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    pub fn balance_of(&self, address: &AccountHash) -> U256 {
        self.balances.get(address)
    }

    pub fn transfer(&mut self, recipient: AccountHash, amount: U256) {
        self._transfer(runtime::get_caller(), recipient, amount)
    }

    pub fn allowance(&self, owner: AccountHash, spender: AccountHash) -> U256 {
        self.allowances.get(&(owner, spender))
    }

    pub fn approve(&mut self, spender: AccountHash, amount: U256) {
        self._approve(runtime::get_caller(), spender, amount);
    }

    pub fn transfer_from(&mut self, owner: AccountHash, recipient: AccountHash, amount: U256) {
        let spender = runtime::get_caller();
        self._transfer(owner, recipient, amount);
        self._approve(owner, spender, self.allowance(owner, spender) - amount);
    }

    fn _transfer(&mut self, sender: AccountHash, recipient: AccountHash, amount: U256) {
        self.balances
            .set(&sender, self.balances.get(&sender) - amount);
        self.balances
            .set(&recipient, self.balances.get(&recipient) + amount);
    }

    fn _approve(&mut self, owner: AccountHash, spender: AccountHash, amount: U256) {
        self.allowances.set(&(owner, spender), amount);
    }
}

// ----- Constructor -----
#[no_mangle]
pub extern "C" fn new() {
    let token_name: String = runtime::get_named_arg("token_name");
    let token_symbol: String = runtime::get_named_arg("token_symbol");
    let total_supply: U256 = runtime::get_named_arg("total_supply");
    let contract = ERC20::new(token_name, token_symbol, total_supply);
    contract.save();
}

// ----- Public Methods -----
#[no_mangle]
pub extern "C" fn name() {
    let contract = ERC20::default();
    let result = contract.name();
    runtime::ret(types::CLValue::from_t(result).unwrap());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let contract = ERC20::default();
    let result = contract.symbol();
    runtime::ret(types::CLValue::from_t(result).unwrap());
}

#[no_mangle]
pub extern "C" fn totalSupply() {
    let contract = ERC20::default();
    let result = contract.total_supply();
    runtime::ret(types::CLValue::from_t(result).unwrap());
}

#[no_mangle]
pub extern "C" fn balanceOf() {
    let contract = ERC20::default();
    let address: AccountHash = runtime::get_named_arg("address");
    let result = contract.balance_of(&address);
    runtime::ret(types::CLValue::from_t(result).unwrap());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let mut contract = ERC20::default();
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    contract.transfer(recipient, amount);
}

#[no_mangle]
pub extern "C" fn allowance() {
    let contract = ERC20::default();
    let owner: AccountHash = runtime::get_named_arg("owner");
    let spender: AccountHash = runtime::get_named_arg("spender");
    let result = contract.allowance(owner, spender);
    runtime::ret(types::CLValue::from_t(result).unwrap());
}

#[no_mangle]
pub extern "C" fn approve() {
    let mut contract = ERC20::default();
    let spender: AccountHash = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    contract.approve(spender, amount);
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let mut contract = ERC20::default();
    let owner: AccountHash = runtime::get_named_arg("owner");
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    contract.transfer_from(owner, recipient, amount);
}

#[no_mangle]
pub extern "C" fn call() {
    let token_name: String = runtime::get_named_arg("token_name");
    let token_symbol: String = runtime::get_named_arg("token_symbol");
    let total_supply: U256 = runtime::get_named_arg("total_supply");

    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let _constructor_access_uref: types::URef = storage::create_contract_user_group(
        contract_package_hash,
        "constructor_group",
        1,
        BTreeSet::new(),
    )
    .unwrap_or_revert()
    .pop()
    .unwrap_or_revert();
    let constructor_group = types::Group::new("constructor_group");
    let mut entry_points = types::EntryPoints::new();

    entry_points.add_entry_point(types::EntryPoint::new(
        String::from("new"),
        vec![
            types::Parameter::new("token_name", types::CLType::String),
            types::Parameter::new("token_symbol", types::CLType::String),
            types::Parameter::new("total_supply", types::CLType::U256),
        ],
        types::CLType::Unit,
        types::EntryPointAccess::Groups(vec![constructor_group]),
        types::EntryPointType::Contract,
    ));

    entry_points.add_entry_point(types::EntryPoint::new(
        String::from("transfer"),
        vec![
            types::Parameter::new("recipient", AccountHash::cl_type()),
            types::Parameter::new("amount", U256::cl_type()),
        ],
        types::CLType::Unit,
        types::EntryPointAccess::Public,
        types::EntryPointType::Contract,
    ));

    entry_points.add_entry_point(types::EntryPoint::new(
        String::from("approve"),
        vec![
            types::Parameter::new("spender", AccountHash::cl_type()),
            types::Parameter::new("amount", U256::cl_type()),
        ],
        types::CLType::Unit,
        types::EntryPointAccess::Public,
        types::EntryPointType::Contract,
    ));

    entry_points.add_entry_point(types::EntryPoint::new(
        String::from("transfer_from"),
        vec![
            types::Parameter::new("owner", AccountHash::cl_type()),
            types::Parameter::new("recipient", AccountHash::cl_type()),
            types::Parameter::new("amount", U256::cl_type()),
        ],
        types::CLType::Unit,
        types::EntryPointAccess::Public,
        types::EntryPointType::Contract,
    ));

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, Default::default());
    runtime::put_key("ERC20", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("ERC20_hash", contract_hash_pack.into());
    runtime::call_contract::<()>(
        contract_hash,
        "new",
        types::runtime_args! {
            "token_name" => token_name,
            "token_symbol" => token_symbol,
            "total_supply" => total_supply
        },
    );
}
