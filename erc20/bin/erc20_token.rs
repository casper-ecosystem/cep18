#![no_main]

use casper_types::U256;
use contract_utils::{BlockchainContext, Context};
use erc20::{self, data, set_erc20_instance, ERC20};

const INITIAL_SUPPLY: u32 = 1_000;

struct Token;
impl BlockchainContext for Token {}
impl ERC20 for Token {
    // Override the constructor to mint tokens at start.
    fn constructor(&self, name: String, symbol: String, decimals: u8) {
        data::init_data(name, symbol, decimals);
        self.mint(self.get_caller(), U256::from(INITIAL_SUPPLY));
    }
}

#[no_mangle]
fn init_erc20() {
    set_erc20_instance(Box::new(Token));
}

#[no_mangle]
fn call() {
    erc20::install_new();
}
