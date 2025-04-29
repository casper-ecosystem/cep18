//! Implementation of balances.
use crate::{
    constants::DICT_BALANCES,
    error::Cep18Error,
    utils::{base64_encode, get_dictionary_value_from_key, set_dictionary_value_for_key},
};
use alloc::string::String;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{bytesrepr::ToBytes, Key, U256};

/// Creates a dictionary item key for a dictionary item, by base64 encoding the Key argument
/// since stringified Keys are too long to be used as dictionary keys.
#[inline]
/// ! TODO GR check hex::encode with utils::make_dictionary_item_key or else
fn make_dictionary_item_key(owner: Key) -> String {
    let preimage = owner
        .to_bytes()
        .unwrap_or_revert_with(Cep18Error::FailedToConvertBytes);
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is 33 bytes for
    // both used Key variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    base64_encode(preimage)
}

/// Writes token balance of a specified account into a dictionary.
pub fn write_balance_to(address: Key, amount: U256) {
    let dictionary_item_key = make_dictionary_item_key(address);
    set_dictionary_value_for_key(DICT_BALANCES, &dictionary_item_key, &amount)
}

/// Reads token balance of a specified account.
///
/// If a given account does not have balances in the system, then a 0 is returned.
pub fn read_balance_from(address: Key) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(address);
    get_dictionary_value_from_key(DICT_BALANCES, &dictionary_item_key).unwrap_or_default()
}

/// Transfer tokens from the `sender` to the `recipient`.
///
/// This function does not validate the sender nor recipient. Check sender and recipient before
/// using this function.
pub fn transfer_balance(sender: Key, recipient: Key, amount: U256) -> Result<(), Cep18Error> {
    if sender == recipient || amount.is_zero() {
        return Ok(());
    }
    let new_sender_balance = {
        let sender_balance = read_balance_from(sender);
        sender_balance
            .checked_sub(amount)
            .ok_or(Cep18Error::InsufficientBalance)?
    };

    let new_recipient_balance = {
        let recipient_balance = read_balance_from(recipient);
        recipient_balance
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)?
    };

    write_balance_to(sender, new_sender_balance);
    write_balance_to(recipient, new_recipient_balance);

    Ok(())
}
