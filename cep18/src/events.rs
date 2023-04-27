use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{Key, U256};

use crate::{
    constants::{BURN_ENTRY_POINT_NAME, MINT_ENTRY_POINT_NAME, OWNER, RECIPIENT, SPENDER},
    utils::get_package_hash,
};

pub const PREFIX_HASH_KEY_NAME: &str = "cep18_package";
pub const EVENT_TYPE: &str = "event_type";
pub const TOKEN_AMOUNT: &str = "token_amount";
pub const LEN: &str = "len";
pub const EVENTS: &str = "events";

pub enum Event {
    Mint {
        recipient: Key,
        amount: U256,
    },
    Burn {
        owner: Key,
        amount: U256,
    },
    SetAllowance {
        owner: Key,
        spender: Key,
        amount: U256,
    },
    IncreaseAllowance {
        owner: Key,
        spender: Key,
        amount: U256,
    },
    DecreaseAllowance {
        owner: Key,
        spender: Key,
        amount: U256,
    },
    Transfer {
        sender: Key,
        recipient: Key,
        amount: U256,
    },
}

pub fn record_event_dictionary(event: Event) {
    let package = get_package_hash().to_string();
    let event: BTreeMap<&str, String> = match event {
        Event::Mint {
            recipient,
            amount: token_amount,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, MINT_ENTRY_POINT_NAME.to_string());
            event.insert(RECIPIENT, recipient.to_string());
            event.insert(TOKEN_AMOUNT, token_amount.to_string());
            event
        }
        Event::Burn {
            owner,
            amount: token_amount,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, BURN_ENTRY_POINT_NAME.to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(TOKEN_AMOUNT, token_amount.to_string());
            event
        }
        Event::SetAllowance {
            owner,
            spender,
            amount: token_amount,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "SetAllowance".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(SPENDER, spender.to_string());
            event.insert(TOKEN_AMOUNT, token_amount.to_string());
            event
        }
        Event::IncreaseAllowance {
            owner,
            spender,
            amount: token_amount,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "IncreaseAllowance".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(SPENDER, spender.to_string());
            event.insert(TOKEN_AMOUNT, token_amount.to_string());
            event
        }
        Event::DecreaseAllowance {
            owner,
            spender,
            amount: token_amount,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "DecreaseAllowance".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(SPENDER, spender.to_string());
            event.insert(TOKEN_AMOUNT, token_amount.to_string());
            event
        }
        Event::Transfer {
            sender,
            recipient,
            amount: token_amount,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "Transfer".to_string());
            event.insert(OWNER, sender.to_string());
            event.insert(RECIPIENT, recipient.to_string());
            event.insert(TOKEN_AMOUNT, token_amount.to_string());
            event
        }
    };
    let dictionary_uref = match runtime::get_key(EVENTS) {
        Some(dict_uref) => dict_uref.into_uref().unwrap_or_revert(),
        None => storage::new_dictionary(EVENTS).unwrap_or_revert(),
    };
    let len = storage::dictionary_get(dictionary_uref, LEN)
        .unwrap_or_revert()
        .unwrap_or(0_u64);
    storage::dictionary_put(dictionary_uref, &len.to_string(), event);
    storage::dictionary_put(dictionary_uref, LEN, len + 1);
}
