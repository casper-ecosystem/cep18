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
mod stakes;
mod total_supply;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, EntryPoints, Key, URef, U256};

use crate::balances::{read_balance_from, write_balance_to};
pub use address::Address;
use constants::{
    ALLOWANCES_KEY_NAME, BALANCES_KEY_NAME, DECIMALS_KEY_NAME, ERC20_TOKEN_CONTRACT_KEY_NAME,
    NAME_KEY_NAME, REWARDS_KEY_NAME, STAKERS_KEY_NAME, STAKES_KEY_NAME, SYMBOL_KEY_NAME,
    TOTAL_SUPPLY_KEY_NAME,
};
pub use error::Error;
use stakes::add_staker;

/// Implementation of ERC20 standard functionality.
#[derive(Default)]
pub struct ERC20 {
    balances_uref: OnceCell<URef>,
    allowances_uref: OnceCell<URef>,
    total_supply_uref: OnceCell<URef>,
    stakers_uref: OnceCell<URef>,
    stakes_uref: OnceCell<URef>,
    rewards_uref: OnceCell<URef>,
}

impl ERC20 {
    fn new(
        balances_uref: URef,
        allowances_uref: URef,
        total_supply_uref: URef,
        stakers_uref: URef,
        stakes_uref: URef,
        rewards_uref: URef,
    ) -> Self {
        Self {
            balances_uref: balances_uref.into(),
            allowances_uref: allowances_uref.into(),
            total_supply_uref: total_supply_uref.into(),
            stakers_uref: stakers_uref.into(),
            stakes_uref: stakes_uref.into(),
            rewards_uref: rewards_uref.into(),
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

    /// Staking

    fn stakers_uref(&self) -> URef {
        *self.stakers_uref.get_or_init(stakes::stakers_uref)
    }

    fn stakes_uref(&self) -> URef {
        *self.stakes_uref.get_or_init(stakes::stakes_uref)
    }

    fn rewards_uref(&self) -> URef {
        *self.rewards_uref.get_or_init(stakes::rewards_uref)
    }

    fn read_stakers(&self) -> Vec<Address> {
        stakes::read_stakers_from(self.stakers_uref())
    }

    ///
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

    /// Creates stake for an account.
    pub fn create_stake(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
        let new_owner_balance = {
            let owner_balance = read_balance_from(self.balances_uref(), owner);
            owner_balance
                .checked_sub(amount)
                .ok_or(Error::InsufficientBalance)?
        };

        let new_stake_balance = {
            let stake_balance = read_balance_from(self.stakes_uref(), owner);
            stake_balance.checked_add(amount).ok_or(Error::Overflow)?
        };

        add_staker(self.stakers_uref(), owner);
        write_balance_to(self.balances_uref(), owner, new_owner_balance);
        write_balance_to(self.stakes_uref(), owner, new_stake_balance);

        Ok(())
    }

    /// Removes stake of an account.
    pub fn remove_stake(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
        let new_stake_balance = {
            let stake_balance = read_balance_from(self.stakes_uref(), owner);
            stake_balance
                .checked_sub(amount)
                .ok_or(Error::InsufficientBalance)?
        };

        let new_owner_balance = {
            let owner_balance = read_balance_from(self.balances_uref(), owner);
            owner_balance.checked_add(amount).ok_or(Error::Overflow)?
        };

        write_balance_to(self.balances_uref(), owner, new_owner_balance);
        write_balance_to(self.stakes_uref(), owner, new_stake_balance);
        Ok(())
    }

    /// Returns stake of an account.
    pub fn stake_of(&mut self, owner: Address) -> Result<U256, Error> {
        Ok(stakes::read_stake_from(self.stakes_uref(), owner))
    }

    /// Returns total amount staked.
    pub fn total_stakes(&mut self) -> Result<U256, Error> {
        let mut amount: U256 = U256::zero();
        let stakers: Vec<Address> = stakes::read_stakers_from(self.stakers_uref());

        for s in stakers {
            amount += stakes::read_stake_from(self.stakes_uref(), s);
        }

        Ok(amount)
    }

    /// Returns true if is a staker, false otherwise.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn is_staker(&mut self, owner: Address) -> Result<bool, Error> {
        for s in self.read_stakers() {
            if s == owner {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Adds a staker.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn add_staker(&mut self, owner: Address) -> Result<(), Error> {
        stakes::add_staker(self.stakers_uref(), owner);

        Ok(())
    }

    /// Returns rewards of a staker.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn reward_of(&mut self, owner: Address) -> Result<U256, Error> {
        Ok(stakes::read_reward_from(self.rewards_uref(), owner))
    }

    /// Returns total amount of rewards.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn total_rewards(&mut self) -> Result<U256, Error> {
        let mut amount: U256 = U256::zero();

        let stakers: Vec<Address> = stakes::read_stakers_from(self.stakers_uref());

        for s in stakers {
            amount += stakes::read_reward_from(self.rewards_uref(), s);
        }

        Ok(amount)
    }

    /// Calculates reward for a staker.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn calculate_rewards(&mut self, owner: Address) -> Result<U256, Error> {
        let staked = stakes::read_stake_from(self.stakes_uref(), owner);
        let (reward, _) = staked.div_mod(U256::from(10)); // 10% of the staked amount without remainder

        Ok(reward)
    }

    /// Distribute rewards to all stakers
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn distribute_rewards(&mut self) -> Result<(), Error> {
        let stakers: Vec<Address> = stakes::read_stakers_from(self.stakers_uref());

        for s in stakers {
            let mut total_reward = stakes::read_reward_from(self.rewards_uref(), s);
            let current_reward = self.calculate_rewards(s).ok().unwrap();
            total_reward = total_reward
                .checked_add(current_reward)
                .ok_or(Error::Overflow)?;

            stakes::write_reward_to(self.rewards_uref(), s, total_reward);
        }

        Ok(())
    }

    /// Withdraw reward
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn withdraw_reward(&mut self, owner: Address) -> Result<(), Error> {
        let reward_balance = read_balance_from(self.rewards_uref(), owner);

        let new_owner_balance = {
            let owner_balance = read_balance_from(self.balances_uref(), owner);
            owner_balance
                .checked_add(reward_balance)
                .ok_or(Error::Overflow)?
        };

        write_balance_to(self.balances_uref(), owner, new_owner_balance);
        write_balance_to(self.rewards_uref(), owner, U256::zero());

        let new_total_supply = {
            let total_supply: U256 = self.read_total_supply();
            total_supply
                .checked_add(reward_balance)
                .ok_or(Error::Overflow)?
        };
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

        let stakers_uref = storage::new_dictionary(STAKERS_KEY_NAME).unwrap_or_revert();
        let stakes_uref = storage::new_dictionary(STAKES_KEY_NAME).unwrap_or_revert();
        let rewards_uref = storage::new_dictionary(REWARDS_KEY_NAME).unwrap_or_revert();

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

        let stakers_dictionary_key = {
            runtime::remove_key(STAKERS_KEY_NAME);

            Key::from(stakers_uref)
        };

        let stakes_dictionary_key = {
            runtime::remove_key(STAKES_KEY_NAME);

            Key::from(stakes_uref)
        };

        let rewards_dictionary_key = {
            runtime::remove_key(REWARDS_KEY_NAME);

            Key::from(rewards_uref)
        };

        named_keys.insert(NAME_KEY_NAME.to_string(), name_key);
        named_keys.insert(SYMBOL_KEY_NAME.to_string(), symbol_key);
        named_keys.insert(DECIMALS_KEY_NAME.to_string(), decimals_key);
        named_keys.insert(BALANCES_KEY_NAME.to_string(), balances_dictionary_key);
        named_keys.insert(ALLOWANCES_KEY_NAME.to_string(), allowances_dictionary_key);
        named_keys.insert(STAKERS_KEY_NAME.to_string(), stakers_dictionary_key);
        named_keys.insert(STAKES_KEY_NAME.to_string(), stakes_dictionary_key);
        named_keys.insert(REWARDS_KEY_NAME.to_string(), rewards_dictionary_key);

        named_keys.insert(TOTAL_SUPPLY_KEY_NAME.to_string(), total_supply_key);

        let (contract_hash, _version) =
            storage::new_locked_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC20::new(
            balances_uref,
            allowances_uref,
            total_supply_uref,
            stakers_uref,
            stakes_uref,
            rewards_uref,
        ))
    }
}
