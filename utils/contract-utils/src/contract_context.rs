use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, Key};

pub trait ContractContext {
    fn get_caller(&self) -> Key {
        let call_stack = runtime::get_call_stack();
        let caller = call_stack.get(call_stack.len() - 2);
        element_to_key(caller.unwrap_or_revert())
    }

    fn self_addr(&self) -> Key {
        element_to_key(runtime::get_call_stack().last().unwrap_or_revert())
    }
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
