use alloc::vec::Vec;
use alloc::string::{String, ToString};
use core::convert::TryInto;

use casperlabs_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{account::PublicKey, CLValue, URef, U512, bytesrepr::{FromBytes, ToBytes}, CLTyped};

use crate::{api::Api, error::Error, env};
use erc20_logic::{ERC20BurnError, ERC20Trait, ERC20TransferError, ERC20TransferFromError};

pub const TOTAL_SUPPLY_KEY: &str = "total_supply";

pub struct ERC20Token;

impl ERC20Trait<U512, PublicKey> for ERC20Token {
    fn read_balance(&mut self, address: &PublicKey) -> Option<U512> {
        let name = balance_key(address);
        env::key(&name)
    }

    fn save_balance(&mut self, address: &PublicKey, balance: U512) {
        let name = balance_key(address);
        env::set_key(&name, balance);
    }

    fn read_total_supply(&mut self) -> Option<U512> {
        env::key(TOTAL_SUPPLY_KEY)
    }

    fn save_total_supply(&mut self, total_supply: U512) {
        env::set_key(TOTAL_SUPPLY_KEY, total_supply);
    }

    fn read_allowance(&mut self, owner: &PublicKey, spender: &PublicKey) -> Option<U512> {
        let name = allowance_key(owner, spender);
        env::key(&name)
    }

    fn save_allowance(&mut self, owner: &PublicKey, spender: &PublicKey, amount: U512) {
        let name = allowance_key(owner, spender);
        env::set_key(&name, amount);
    }
}

fn balance_key(public_key: &PublicKey) -> String {
    public_key.to_string()
}

fn allowance_key(owner: &PublicKey, spender: &PublicKey) -> String {
    format!("{}{}", owner, spender)
}


