use core::convert::TryInto;

use crate::{
    api::{self},
    error::Error,
};
use casperlabs_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, Key, U512,
};

pub const ERC20_CONTRACT_NAME: &str = "erc20";
pub const ERC20_PROXY_CONTRACT_NAME: &str = "erc20_proxy";

pub fn deploy_token(initial_balance: U512) {
    let token_ref = storage::store_function_at_hash(ERC20_CONTRACT_NAME, Default::default());
    runtime::call_contract::<_, ()>(token_ref.clone(), (api::INIT_ERC20, initial_balance));
    let contract_key: Key = token_ref.into();
    let token: Key = storage::new_uref(contract_key).into();
    runtime::put_key(ERC20_CONTRACT_NAME, token);
}

pub fn deploy_proxy() {
    let proxy_ref = storage::store_function_at_hash(ERC20_PROXY_CONTRACT_NAME, Default::default());
    let contract_key: Key = proxy_ref.into();
    let proxy: Key = storage::new_uref(contract_key).into();
    runtime::put_key(ERC20_PROXY_CONTRACT_NAME, proxy);
}

pub fn key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(maybe_key) => {
            let key = maybe_key
                .try_into()
                .unwrap_or_revert_with(Error::UnexpectedType);
            let value = storage::read(key)
                .unwrap_or_revert_with(Error::MissingKey)
                .unwrap_or_revert_with(Error::UnexpectedType);
            Some(value)
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn is_initialized(name: &str) -> bool {
    key::<bool>(name).is_some()
}

pub fn mark_as_initialized(name: &str) {
    set_key(name, true);
}
