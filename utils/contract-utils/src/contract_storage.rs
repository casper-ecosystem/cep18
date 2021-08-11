use alloc::{sync::Arc, vec::Vec};
use spin::Mutex;

use casper_contract::contract_api::runtime;
use casper_types::system::CallStackElement;

pub struct ContractStorage {
    call_stack: Arc<Mutex<Option<Arc<Vec<CallStackElement>>>>>,
}

impl Default for ContractStorage {
    fn default() -> Self {
        ContractStorage {
            call_stack: Arc::new(Mutex::new(None)),
        }
    }
}

impl ContractStorage {
    pub fn call_stack(&self) -> Arc<Vec<CallStackElement>> {
        let mut call_stack_wrapper = self.call_stack.lock();
        call_stack_wrapper.clone().unwrap_or_else(|| {
            let call_stack = Arc::new(runtime::get_call_stack());
            *call_stack_wrapper = Some(call_stack.clone());
            call_stack
        })
    }
}
