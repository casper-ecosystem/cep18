#![no_main]

use casper_dsl::dsl::*;
use casper_dsl::types::{account::AccountHash, U256};

#[derive(Context)]
struct ERC20 {
    token_name: Variable<String>,
    token_symbol: Variable<String>,
    total_supply: Variable<U256>,
    balances: Map<AccountHash, U256>,
    allowances: Map<(AccountHash, AccountHash), U256>,
}

#[casper_contract]
impl ERC20 {
    #[casper_constructor]
    pub fn new(token_name: String, token_symbol: String, total_supply: U256) -> ERC20 {
        let mut erc20 = ERC20::default();
        erc20.balances.set(&runtime::get_caller(), total_supply);
        *erc20.total_supply = total_supply;
        *erc20.token_name = token_name;
        *erc20.token_symbol = token_symbol;
        erc20
    }

    #[casper_method]
    pub fn name(&self) -> String {
        self.token_name.get()
    }

    #[casper_method]
    pub fn symbol(&self) -> String {
        self.token_symbol.get()
    }

    #[casper_method]
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    #[casper_method]
    pub fn balance_of(&self, address: AccountHash) -> U256 {
        self.balances.get(&address)
    }

    #[casper_method]
    pub fn transfer(&mut self, recipient: AccountHash, amount: U256) {
        self._transfer(runtime::get_caller(), recipient, amount)
    }

    #[casper_method]
    pub fn allowance(&self, owner: AccountHash, spender: AccountHash) -> U256 {
        self.allowances.get(&(owner, spender))
    }

    #[casper_method]
    pub fn approve(&mut self, spender: AccountHash, amount: U256) {
        self._approve(runtime::get_caller(), spender, amount);
    }

    #[casper_method]
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
