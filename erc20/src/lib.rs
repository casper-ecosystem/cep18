//! A library for developing ERC20 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC20`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.
//!
//! To create an example ERC20 contract which uses this library, use the cargo-casper tool:
//!
//! ```bash
//! cargo install cargo-casper
//! cargo casper --erc20 <PATH TO NEW PROJECT>
//! ```

#![warn(missing_docs)]
#![no_std]

extern crate alloc;

mod address;
mod allowances;
mod balances;
pub mod constants;
mod detail;
pub mod entry_points;
mod error;
mod total_supply;

use alloc::string::{String, ToString};

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, EntryPoints, Key, URef, U256};

pub use address::Address;
use constants::{
    ALLOWANCES_KEY_NAME, BALANCES_KEY_NAME, DECIMALS_KEY_NAME, ERC20_TOKEN_CONTRACT_KEY_NAME,
    NAME_KEY_NAME, SYMBOL_KEY_NAME, TOTAL_SUPPLY_KEY_NAME,
};
pub use error::Error;

/// Implementation of ERC20 standard functionality.
#[derive(Default)]
pub struct ERC20 {
    balances_uref: OnceCell<URef>,
    allowances_uref: OnceCell<URef>,
    total_supply_uref: OnceCell<URef>,
}

impl ERC20 {
    fn new(balances_uref: URef, allowances_uref: URef, total_supply_uref: URef) -> Self {
        Self {
            balances_uref: balances_uref.into(),
            allowances_uref: allowances_uref.into(),
            total_supply_uref: total_supply_uref.into(),
        }
    }

    fn total_supply_uref(&self) -> URef {
        *self
            .total_supply_uref
            .get_or_init(total_supply::total_supply_uref)
    }

    fn read_total_supply(&self) -> U256 {
        total_supply::read_total_supply_from(self.total_supply_uref())
    }

    fn write_total_supply(&self, total_supply: U256) {
        total_supply::write_total_supply_to(self.total_supply_uref(), total_supply)
    }

    fn balances_uref(&self) -> URef {
        *self.balances_uref.get_or_init(balances::get_balances_uref)
    }

    fn read_balance(&self, owner: Address) -> U256 {
        balances::read_balance_from(self.balances_uref(), owner)
    }

    fn write_balance(&mut self, owner: Address, amount: U256) {
        balances::write_balance_to(self.balances_uref(), owner, amount)
    }

    fn allowances_uref(&self) -> URef {
        *self
            .allowances_uref
            .get_or_init(allowances::allowances_uref)
    }

    fn read_allowance(&self, owner: Address, spender: Address) -> U256 {
        allowances::read_allowance_from(self.allowances_uref(), owner, spender)
    }

    fn write_allowance(&mut self, owner: Address, spender: Address, amount: U256) {
        allowances::write_allowance_to(self.allowances_uref(), owner, spender, amount)
    }

    fn transfer_balance(
        &mut self,
        sender: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        balances::transfer_balance(self.balances_uref(), sender, recipient, amount)
    }

    /// Installs the ERC20 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install(
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
    ) -> Result<ERC20, Error> {
        let default_entry_points = entry_points::default();
        ERC20::install_custom(
            name,
            symbol,
            decimals,
            initial_supply,
            ERC20_TOKEN_CONTRACT_KEY_NAME,
            default_entry_points,
        )
    }

    /// Returns the name of the token.
    pub fn name(&self) -> String {
        detail::read_from(NAME_KEY_NAME)
    }

    /// Returns the symbol of the token.
    pub fn symbol(&self) -> String {
        detail::read_from(SYMBOL_KEY_NAME)
    }

    /// Returns the decimals of the token.
    pub fn decimals(&self) -> u8 {
        detail::read_from(DECIMALS_KEY_NAME)
    }

    /// Returns the total supply of the token.
    pub fn total_supply(&self) -> U256 {
        self.read_total_supply()
    }

    /// Returns the balance of `owner`.
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.read_balance(owner)
    }

    /// Transfers `amount` of tokens from the direct caller to `recipient`.
    pub fn transfer(&mut self, recipient: Address, amount: U256) -> Result<(), Error> {
        let sender = detail::get_immediate_caller_address()?;
        self.transfer_balance(sender, recipient, amount)
    }

    /// Transfers `amount` of tokens from `owner` to `recipient` if the direct caller has been
    /// previously approved to spend the specified amount on behalf of the owner.
    pub fn transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        let spender = detail::get_immediate_caller_address()?;
        if amount.is_zero() {
            return Ok(());
        }
        let spender_allowance = self.read_allowance(owner, spender);
        let new_spender_allowance = spender_allowance
            .checked_sub(amount)
            .ok_or(Error::InsufficientAllowance)?;
        self.transfer_balance(owner, recipient, amount)?;
        self.write_allowance(owner, spender, new_spender_allowance);
        Ok(())
    }

    /// Allows `spender` to transfer up to `amount` of the direct caller's tokens.
    pub fn approve(&mut self, spender: Address, amount: U256) -> Result<(), Error> {
        let owner = detail::get_immediate_caller_address()?;
        self.write_allowance(owner, spender, amount);
        Ok(())
    }

    /// Returns the amount of `owner`'s tokens allowed to be spent by `spender`.
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.read_allowance(owner, spender)
    }

    /// Mints `amount` new tokens and adds them to `owner`'s balance and to the token total supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn mint(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
        let new_balance = {
            let balance = self.read_balance(owner);
            balance.checked_add(amount).ok_or(Error::Overflow)?
        };
        let new_total_supply = {
            let total_supply: U256 = self.read_total_supply();
            total_supply.checked_add(amount).ok_or(Error::Overflow)?
        };
        self.write_balance(owner, new_balance);
        self.write_total_supply(new_total_supply);
        Ok(())
    }

    /// Burns (i.e. subtracts) `amount` of tokens from `owner`'s balance and from the token total
    /// supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn burn(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
        let new_balance = {
            let balance = self.read_balance(owner);
            balance
                .checked_sub(amount)
                .ok_or(Error::InsufficientBalance)?
        };
        let new_total_supply = {
            let total_supply = self.read_total_supply();
            total_supply.checked_sub(amount).ok_or(Error::Overflow)?
        };
        self.write_balance(owner, new_balance);
        self.write_total_supply(new_total_supply);
        Ok(())
    }

    /// Installs the ERC20 contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC20::install`] instead, as it will create the default set
    /// of ERC20 entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC20, Error> {
        let balances_uref = storage::new_dictionary(BALANCES_KEY_NAME).unwrap_or_revert();
        let allowances_uref = storage::new_dictionary(ALLOWANCES_KEY_NAME).unwrap_or_revert();
        // We need to hold on a RW access rights because tokens can be minted or burned.
        let total_supply_uref = storage::new_uref(initial_supply).into_read_write();

        let mut named_keys = NamedKeys::new();

        let name_key = {
            let name_uref = storage::new_uref(name).into_read();
            Key::from(name_uref)
        };

        let symbol_key = {
            let symbol_uref = storage::new_uref(symbol).into_read();
            Key::from(symbol_uref)
        };

        let decimals_key = {
            let decimals_uref = storage::new_uref(decimals).into_read();
            Key::from(decimals_uref)
        };

        let total_supply_key = Key::from(total_supply_uref);

        let balances_dictionary_key = {
            // Sets up initial balance for the caller - either an account, or a contract.
            let caller = detail::get_caller_address()?;
            balances::write_balance_to(balances_uref, caller, initial_supply);

            runtime::remove_key(BALANCES_KEY_NAME);

            Key::from(balances_uref)
        };

        let allowances_dictionary_key = {
            runtime::remove_key(ALLOWANCES_KEY_NAME);

            Key::from(allowances_uref)
        };

        named_keys.insert(NAME_KEY_NAME.to_string(), name_key);
        named_keys.insert(SYMBOL_KEY_NAME.to_string(), symbol_key);
        named_keys.insert(DECIMALS_KEY_NAME.to_string(), decimals_key);
        named_keys.insert(BALANCES_KEY_NAME.to_string(), balances_dictionary_key);
        named_keys.insert(ALLOWANCES_KEY_NAME.to_string(), allowances_dictionary_key);
        named_keys.insert(TOTAL_SUPPLY_KEY_NAME.to_string(), total_supply_key);

        let (contract_hash, _version) =
            storage::new_locked_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC20::new(
            balances_uref,
            allowances_uref,
            total_supply_uref,
        ))
    }
}
