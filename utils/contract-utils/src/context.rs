use alloc::vec::Vec;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, Key};
use lazy_static::lazy_static;

lazy_static! {
    static ref CALL_STACK: Vec<CallStackElement> = runtime::get_call_stack();
}

fn element_to_key(element: &CallStackElement) -> Key {
    match element {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => (*contract_package_hash).into(),
    }
}

pub trait Context: Send + Sync {
    fn get_caller(&self) -> Key;
}

pub trait BlockchainContext: Send + Sync {}

impl<T: BlockchainContext> Context for T {
    fn get_caller(&self) -> Key {
        let call_stack = &*CALL_STACK;
        let caller = call_stack.get(call_stack.len() - 2);
        element_to_key(caller.unwrap_or_revert())
    }
}
