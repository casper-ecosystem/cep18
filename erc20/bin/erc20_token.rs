#![no_main]

use contract_utils::BlockchainContext;
use erc20::{self, set_erc20_instance, SimpleERC20};

struct SimpleToken;
impl BlockchainContext for SimpleToken {}
impl SimpleERC20 for SimpleToken {}

#[no_mangle]
fn init_erc20() {
    set_erc20_instance(Box::new(SimpleToken));
}

#[no_mangle]
fn call() {
    erc20::install_new();
}
