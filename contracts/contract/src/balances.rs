//! Implementation of balances.
use crate::{
    constants::DICT_BALANCES,
    error::Cep18Error,
    utils::{
        get_dictionary_value_from_key, make_dictionary_item_key, set_dictionary_value_for_key,
        key_as_account_or_package,
    },
};
use casper_types::{Key, U256};

/// Writes token balance of a specified account into a dictionary.
pub fn write_balance_to(address: Key, amount: U256) {
    let address = key_as_account_or_package(address);
    let dictionary_item_key = make_dictionary_item_key(address);
    set_dictionary_value_for_key(DICT_BALANCES, &dictionary_item_key, &amount)
}

/// Reads token balance of a specified account.
///
/// If a given account does not have balances in the system, then a 0 is returned.
pub fn read_balance_from(address: Key) -> U256 {
    let address = key_as_account_or_package(address);
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
