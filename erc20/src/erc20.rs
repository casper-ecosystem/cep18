use casper_types::{Key, U256};
use contract_utils::Context;

use crate::data::{self, Allowances, Balances};

pub trait ERC20: Context + Send + Sync {
    fn constructor(&self, name: String, symbol: String, decimals: u8) {
        data::init_data(name, symbol, decimals);
    }

    fn balance_of(&self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }

    fn transfer(&self, recipient: Key, amount: U256) {
        self.make_transfer(self.get_caller(), recipient, amount);
    }

    fn approve(&self, spender: Key, amount: U256) {
        Allowances::instance().set(&self.get_caller(), &spender, amount);
    }

    fn allowance(&self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn transfer_from(&self, owner: Key, recipient: Key, amount: U256) {
        let allowances = Allowances::instance();
        let spender = self.get_caller();
        let spender_allowance = allowances.get(&owner, &spender);
        allowances.set(&owner, &spender, spender_allowance - amount);
        self.make_transfer(owner, recipient, amount);
    }

    fn mint(&self, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let balance = balances.get(&recipient);
        balances.set(&recipient, balance + amount);

        data::set_total_supply(data::total_supply() + amount);
    }

    fn make_transfer(&self, sender: Key, recipient: Key, amount: U256) {
        let balances = Balances::instance();

        let sender_balance = balances.get(&sender);
        let recipient_balance = balances.get(&recipient);

        balances.set(&sender, sender_balance - amount);
        balances.set(&recipient, recipient_balance + amount);
    }

    fn total_supply(&self) -> U256 {
        data::total_supply()
    }
}
