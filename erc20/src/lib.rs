#![feature(once_cell)]

use casper_contract::contract_api::runtime;
use casper_types::ApiError;
use once_cell::sync::OnceCell;

pub mod data;
mod entrypoints;
mod erc20;
mod install;

pub use entrypoints::get_entry_points;
pub use erc20::ERC20;
pub use install::install_new;

pub static ERC20_INSTANCE: OnceCell<Box<dyn ERC20>> = OnceCell::new();

extern "C" {
    pub fn init_erc20();
}

pub fn set_erc20_instance(instance: Box<dyn ERC20>) {
    if ERC20_INSTANCE.set(instance).is_err() {
        runtime::revert(ApiError::User(1));
    }
}
