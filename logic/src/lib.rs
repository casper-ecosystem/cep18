#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate num_traits;

use core::ops::{Add, Sub};
pub mod errors;

pub use errors::*;

pub trait ERC20Trait<
    Amount: num_traits::Zero + Add<Output = Amount> + Sub<Output = Amount> + PartialOrd + Copy,
    Address,
> 
{
    fn read_balance(&mut self, address: &Address) -> Option<Amount>;
    fn save_balance(&mut self, address: &Address, balance: Amount);
    fn read_total_supply(&mut self) -> Option<Amount>;
    fn save_total_supply(&mut self, total_supply: Amount);
    fn read_allowance(&mut self, owner: &Address, spender: &Address) -> Option<Amount>;
    fn save_allowance(&mut self, owner: &Address, spender: &Address, amount: Amount);

    fn mint(&mut self, address: &Address, amount: Amount) {
        let address_balance = self.balance_of(address);
        let total_supply = self.total_supply();
        self.save_balance(&address, address_balance + amount);
        self.save_total_supply(total_supply + amount);
    }

    fn burn(&mut self, address: &Address, amount: Amount) -> Result<(), ERC20BurnError> {
        let address_balance = self.balance_of(address);
        if address_balance < amount {
            Err(ERC20BurnError::NotEnoughBalance)
        } else {
            let total_supply = self.total_supply();
            self.save_balance(&address, address_balance - amount);
            self.save_total_supply(total_supply - amount);
            Ok(())
        }
    }

    fn transfer(
        &mut self,
        sender: &Address,
        recipient: &Address,
        amount: Amount,
    ) -> Result<(), ERC20TransferError> {
        let sender_balance = self.balance_of(sender);
        if amount > sender_balance {
            Err(ERC20TransferError::NotEnoughBalance)
        } else {
            let recipient_balance = self.balance_of(recipient);
            self.save_balance(&sender, sender_balance - amount);
            self.save_balance(&recipient, recipient_balance + amount);
            Ok(())
        }
    }

    fn balance_of(&mut self, address: &Address) -> Amount {
        self.read_balance(address).unwrap_or_else(Amount::zero)
    }

    fn total_supply(&mut self) -> Amount {
        self.read_total_supply().unwrap_or_else(Amount::zero)
    }

    fn allowance(&mut self, owner: &Address, spender: &Address) -> Amount {
        self.read_allowance(owner, spender).unwrap_or_else(Amount::zero)
    }

    fn approve(&mut self, owner: &Address, spender: &Address, amount: Amount) {
        self.save_allowance(owner, spender, amount)
    }

    fn transfer_from(
        &mut self,
        spender: &Address,
        owner: &Address,
        recipient: &Address,
        amount: Amount,
    ) -> Result<(), ERC20TransferFromError> {
        let allowance = self.allowance(owner, spender);
        if amount > allowance {
            return Err(ERC20TransferFromError::NotEnoughAllowance);
        }
        self.transfer(owner, recipient, amount)?;
        self.approve(owner, spender, allowance - amount);
        Ok(())
    }
}

// Tests start with an example implementation of ERC20Trait.
#[cfg(test)]
mod tests {
    use super::{ERC20BurnError, ERC20Trait, ERC20TransferError, ERC20TransferFromError};

    use std::collections::HashMap;

    type Amount = u64;
    type Address = u8;
    type AddressPair = (Address, Address);

    struct Token {
        total_supply_count: Amount,
        balances: HashMap<Address, Amount>,
        allowance: HashMap<AddressPair, Amount>,
    }

    impl Token {
        fn new() -> Token {
            Token {
                total_supply_count: 0,
                balances: HashMap::new(),
                allowance: HashMap::new(),
            }
        }
    }

    impl ERC20Trait<Amount, Address> for Token {
        fn read_balance(&mut self, address: &Address) -> Option<Amount> {
            self.balances.get(address).cloned()
        }

        fn save_balance(&mut self, address: &Address, balance: Amount) {
            self.balances.insert(*address, balance);
        }

        fn read_total_supply(&mut self) -> Option<Amount> {
            Some(self.total_supply_count)
        }

        fn save_total_supply(&mut self, new_total_supply: Amount) {
            self.total_supply_count = new_total_supply;
        }

        fn read_allowance(&mut self, owner: &Address, spender: &Address) -> Option<Amount> {
            self.allowance.get(&(*owner, *spender)).cloned()
        }

        fn save_allowance(&mut self, owner: &Address, spender: &Address, amount: Amount) {
            self.allowance.insert((*owner, *spender), amount);
        }
    }

    const ADDRESS_1: Address = 1;
    const ADDRESS_2: Address = 2;
    const ADDRESS_3: Address = 3;

    #[test]
    fn test_initial_balances() {
        let mut token = Token::new();
        assert_eq!(token.balance_of(&ADDRESS_1), 0);
        assert_eq!(token.balance_of(&ADDRESS_2), 0);
        assert_eq!(token.total_supply(), 0);
    }

    #[test]
    fn test_mint() {
        let mut token = Token::new();
        token.mint(&ADDRESS_1, 10);
        assert_eq!(token.balance_of(&ADDRESS_1), 10);
        assert_eq!(token.balance_of(&ADDRESS_2), 0);
        assert_eq!(token.total_supply(), 10);
    }

    #[test]
    fn test_burn() {
        let mut token = Token::new();
        token.mint(&ADDRESS_1, 10);
        let result = token.burn(&ADDRESS_1, 5);
        assert!(result.is_ok());
        assert_eq!(token.balance_of(&ADDRESS_1), 5);
        assert_eq!(token.total_supply(), 5);
    }

    #[test]
    fn test_burn_too_much() {
        let mut token = Token::new();
        token.mint(&ADDRESS_1, 10);
        let result = token.burn(&ADDRESS_1, 15);
        assert_eq!(result.unwrap_err(), ERC20BurnError::NotEnoughBalance);
    }

    #[test]
    fn test_transfer() {
        let mut token = Token::new();
        token.mint(&ADDRESS_1, 10);
        let transfer_result = token.transfer(&ADDRESS_1, &ADDRESS_2, 3);
        assert!(transfer_result.is_ok());
        assert_eq!(token.balance_of(&ADDRESS_1), 7);
        assert_eq!(token.balance_of(&ADDRESS_2), 3);
        assert_eq!(token.total_supply(), 10);
    }

    #[test]
    fn test_transfer_too_much() {
        let mut token = Token::new();
        token.mint(&ADDRESS_1, 10);
        let transfer_result = token.transfer(&ADDRESS_1, &ADDRESS_2, 20);
        assert_eq!(
            transfer_result.unwrap_err(),
            ERC20TransferError::NotEnoughBalance
        );
        assert_eq!(token.balance_of(&ADDRESS_1), 10);
        assert_eq!(token.balance_of(&ADDRESS_2), 0);
        assert_eq!(token.total_supply(), 10);
    }

    #[test]
    fn test_initial_allowance() {
        let mut token = Token::new();
        assert_eq!(token.allowance(&ADDRESS_1, &ADDRESS_2), 0);
        assert_eq!(token.allowance(&ADDRESS_2, &ADDRESS_1), 0);
    }

    #[test]
    fn test_approvals() {
        let mut token = Token::new();
        token.approve(&ADDRESS_1, &ADDRESS_2, 10);
        assert_eq!(token.allowance(&ADDRESS_1, &ADDRESS_2), 10);
        assert_eq!(token.allowance(&ADDRESS_2, &ADDRESS_1), 0);
        token.approve(&ADDRESS_1, &ADDRESS_2, 2);
        assert_eq!(token.allowance(&ADDRESS_1, &ADDRESS_2), 2);
        assert_eq!(token.allowance(&ADDRESS_2, &ADDRESS_1), 0);
    }

    #[test]
    fn test_transfer_from() {
        let mut token = Token::new();
        token.mint(&ADDRESS_1, 10);
        token.approve(&ADDRESS_1, &ADDRESS_2, 5);
        let transfer_result = token.transfer_from(&ADDRESS_2, &ADDRESS_1, &ADDRESS_3, 3);
        assert!(transfer_result.is_ok());
        assert_eq!(token.allowance(&ADDRESS_1, &ADDRESS_2), 2);
        assert_eq!(token.balance_of(&ADDRESS_1), 7);
        assert_eq!(token.balance_of(&ADDRESS_2), 0);
        assert_eq!(token.balance_of(&ADDRESS_3), 3);
    }

    #[test]
    fn test_transfer_from_too_much() {
        let mut token = Token::new();
        token.mint(&ADDRESS_1, 10);
        token.approve(&ADDRESS_1, &ADDRESS_2, 25);
        let transfer_result = token.transfer_from(&ADDRESS_2, &ADDRESS_1, &ADDRESS_3, 20);
        let expected_err =
            ERC20TransferFromError::TransferError(ERC20TransferError::NotEnoughBalance);
        assert_eq!(transfer_result.unwrap_err(), expected_err);
        assert_eq!(token.allowance(&ADDRESS_1, &ADDRESS_2), 25);
        assert_eq!(token.balance_of(&ADDRESS_1), 10);
        assert_eq!(token.balance_of(&ADDRESS_2), 0);
        assert_eq!(token.balance_of(&ADDRESS_3), 0);
    }

    #[test]
    fn test_transfer_from_too_low_allowance() {
        let mut token = Token::new();
        token.mint(&ADDRESS_1, 10);
        token.approve(&ADDRESS_1, &ADDRESS_2, 3);
        let transfer_result = token.transfer_from(&ADDRESS_2, &ADDRESS_1, &ADDRESS_3, 5);
        assert_eq!(
            transfer_result.unwrap_err(),
            ERC20TransferFromError::NotEnoughAllowance
        );
        assert_eq!(token.allowance(&ADDRESS_1, &ADDRESS_2), 3);
        assert_eq!(token.balance_of(&ADDRESS_1), 10);
        assert_eq!(token.balance_of(&ADDRESS_2), 0);
        assert_eq!(token.balance_of(&ADDRESS_3), 0);
    }
}
