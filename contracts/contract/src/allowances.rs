//! Implementation of allowances.
use crate::{
    constants::DICT_ALLOWANCES,
    utils::{
        get_dictionary_value_from_key, key_as_account_or_package, make_dictionary_item_key_value,
        set_dictionary_value_for_key,
    },
};
use casper_types::{Key, U256};

/// Writes an allowance for owner and spender for a specific amount.
pub fn write_allowance_to(owner: Key, spender: Key, amount: U256) {
    let owner = key_as_account_or_package(owner);
    let spender = key_as_account_or_package(spender);
    let dictionary_item_key = make_dictionary_item_key_value(&owner, &spender);
    set_dictionary_value_for_key(DICT_ALLOWANCES, &dictionary_item_key, &amount)
}

/// Reads an allowance for a owner and spender
pub fn read_allowance_from(owner: Key, spender: Key) -> U256 {
    let owner = key_as_account_or_package(owner);
    let spender = key_as_account_or_package(spender);
    let dictionary_item_key = make_dictionary_item_key_value(&owner, &spender);
    get_dictionary_value_from_key(DICT_ALLOWANCES, &dictionary_item_key).unwrap_or_default()
}
