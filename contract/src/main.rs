#![no_main]

use dsl::{GetKey, Map, runtime, storage, types::{U256, account::AccountHash}};

struct ERC20 {
    token_name: String, 
    token_symbol: String,
    total_supply: U256,
    balances: Map<AccountHash, U256>,
    allowances: Map<(AccountHash, AccountHash), U256>
}

// #[casper_contract]
impl ERC20 {

    pub fn new(token_name: String, token_symbol: String, total_supply: U256) -> ERC20 {
        let mut erc20 = ERC20::default();
        erc20.balances.set(&runtime::get_caller(), total_supply);
        erc20.total_supply = total_supply;
        erc20.token_name = token_name;
        erc20.token_symbol = token_symbol;
        erc20
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
        self.balances.set(&sender, self.balances.get(&sender) - amount);
        self.balances.set(&recipient, self.balances.get(&recipient) + amount);
    }

    fn _approve(&mut self, owner: AccountHash, spender: AccountHash, amount: U256) {
        self.allowances.set(&(owner, spender), amount);
    }
}
// --------------------------- Generated ------------------------------

// ----- Save -----
use dsl::{Save, UnwrapOrRevert, get_key, set_key};
use types::{CLTyped, RuntimeArgs};
use std::collections::BTreeSet;

impl Save for ERC20 {
    fn save(&self) {
        set_key("token_name", self.token_name.clone());
        set_key("token_symbol", self.token_symbol.clone());
        set_key("total_supply", self.total_supply.clone());
    }
}

impl Default for ERC20 {
    fn default() -> Self {
        ERC20 {
            token_name: get_key("token_name"),
            token_symbol: get_key("token_symbol"),
            total_supply: get_key("total_supply"),
            balances: Map::new(String::from("balances")),
            allowances: Map::new(String::from("allowances"))
        }
    }
}


impl GetKey for AccountHash {
    fn get_key(&self, prefix: &String) -> String {
        format!("{}_{}", prefix, self.to_string())
    }
}

impl GetKey for (AccountHash, AccountHash) {
    fn get_key(&self, prefix: &String) -> String {
        format!("{}_{}_{}", prefix, self.0.to_string(), self.1.to_string())
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
    runtime::call_contract::<()>(contract_hash, "new", types::runtime_args! {
        "token_name" => token_name,
        "token_symbol" => token_symbol,
        "total_supply" => total_supply
    });
}


// --------------------------- DSL ------------------------------------
mod dsl {

    use std::{convert::TryInto, hash::Hash, marker::PhantomData};

    pub use contract::contract_api::runtime;
    pub use contract::contract_api::storage;
    pub use contract::unwrap_or_revert::UnwrapOrRevert;
    pub use types;
    use types::{CLTyped, bytesrepr::{FromBytes, ToBytes}};

    pub struct Map<K, V> {
        prefix: String,
        // storage: HashMap<String, V>,
        key_type: PhantomData<K>,
        value_type: PhantomData<V>
    }

    impl<K, V> Map<K, V> where 
        K: Clone + Default + GetKey + Eq + Hash,
        V: Clone + Default + FromBytes + ToBytes + CLTyped
    {
        pub fn new(prefix: String) -> Self {
            Map {
                prefix: prefix,
                key_type: PhantomData,
                value_type: PhantomData
            }
        }

        pub fn get(&self, key: &K) -> V {
            // self.storage.get(&key.get_key(&self.prefix)).unwrap()
            get_key(&key.get_key(&self.prefix))
        }
    
        pub fn set(&mut self, key: &K, value: V) {
            // self.storage.insert(key.get_key(&self.prefix), value);
            set_key(&key.get_key(&self.prefix), value)
        }
    }

    pub trait Save {
        fn save(&self);
    }

    pub trait GetKey {
        fn get_key(&self, prefix: &String) -> String;
    }

    pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
        match runtime::get_key(name) {
            None => Default::default(),
            Some(value) => {
                let key = value.try_into().unwrap_or_revert();
                storage::read(key).unwrap_or_revert().unwrap_or_revert()
            }
        }
    }
    
    pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
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
    
}
