#![no_main]

use contract_utils::{BlockchainContext, Context};
use erc20::{self, set_erc20_instance, SimpleERC20};

struct SimpleToken;
impl BlockchainContext for SimpleToken {}
impl SimpleERC20 for SimpleToken {
    // Override example.
    fn transfer(&self, recipient: casper_types::Key, amount: casper_types::U256) {
        self.make_transfer(self.get_caller(), recipient, amount);
    }
}

#[no_mangle]
fn init_erc20() {
    set_erc20_instance(Box::new(SimpleToken));
}

#[no_mangle]
fn call() {
    erc20::install_new();
}
