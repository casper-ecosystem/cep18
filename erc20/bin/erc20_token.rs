#![no_main]

use casper_types::{Key, U256};
use contract_interface::contract_interface;
use contract_utils::ContractContext;
use erc20::{self, ERC20};

struct Token {}

impl ContractContext for Token {}
impl ERC20 for Token {}

impl Token {
    fn constructor(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        ERC20::init(self, name, symbol, decimals);
        ERC20::mint(self, self.get_caller(), initial_supply);
    }
}

#[contract_interface(Token)]
trait ERC20Interface {
    fn constructor(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256);
    fn transfer(&mut self, recipient: Key, amount: U256);
    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256);
    fn approve(&mut self, spender: Key, amount: U256);
    fn balance_of(&mut self, owner: Key) -> U256;
    fn allowance(&mut self, owner: Key, spender: Key) -> U256;
    fn total_supply(&mut self) -> U256;
}
