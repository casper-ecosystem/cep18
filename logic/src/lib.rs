#[cfg(test)]
mod tests;

pub mod errors;

use std::ops::{Add, Sub};

pub use errors::{ERC20TransferError, ERC20TransferFromError};

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
        self.read_allowance(owner, spender)
            .unwrap_or_else(Amount::zero)
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
