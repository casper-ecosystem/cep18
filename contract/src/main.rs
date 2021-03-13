#![no_main]

use dsl::{Map, runtime, storage, types::{U256, account::AccountHash}};

#[derive(Default)]
struct ERC20 {
    token_name: String, 
    token_symbol: String,
    total_supply: U256,
    balances: Map<AccountHash, U256>
}

impl ERC20 {

    // #[casper_constructor]
    pub fn new(token_name: String, token_symbol: String, total_supply: U256) -> ERC20 {
        let mut erc20 = ERC20::default();
        erc20.balances.set(&dsl::get_caller(), &total_supply);
        erc20.total_supply = total_supply;
        erc20.token_name = token_name;
        erc20.token_symbol = token_symbol;
        erc20
    }
    
    pub fn balanceOf(&self, address: &AccountHash) -> U256 {
        return self.balances.get(address);
    }
    
    pub fn transfer(&mut self, recipient: &AccountHash, amount: &U256) {
        self.balances.set(recipient, &(self.balances.get(recipient) + amount));
        self.balances.set(&runtime::get_caller(), &(self.balances.get(&runtime::get_caller()) - amount));
        
    }
}
// --------------------------- Generated ------------------------------

// ----- Save -----
use dsl::{Save, UnwrapOrRevert};
use types::{CLTyped, RuntimeArgs};
use std::collections::BTreeSet;

impl Save for ERC20 {
    fn save(&self) {}
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
    let result = contract.balanceOf(&address);
    runtime::ret(types::CLValue::from_t(result).unwrap());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let mut contract = ERC20::default();
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    contract.transfer(&recipient, &amount);
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

    pub use contract::contract_api::runtime;
    pub use contract::contract_api::storage;
    pub use contract::unwrap_or_revert::UnwrapOrRevert;
    pub use types;
    use types::account::AccountHash;

    #[derive(Default)]
    pub struct Map<K: Default, V: Default> {
        k: K,
        v: V
    }

    impl<K: Clone + Default, V: Clone + Default> Map<K, V> {
        pub fn get(&self, key: &K) -> V {
            self.v.clone()
        }
    
        pub fn set(&mut self, key: &K, value: &V) {
            self.k = key.clone();
            self.v = value.clone();
        }
    }

    pub trait Save {
        fn save(&self);
    }

    pub fn get_caller() -> AccountHash {
        runtime::get_caller()
    }

}
