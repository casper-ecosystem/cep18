use core::convert::TryFrom;

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
    constants::{BURN_ENTRY_POINT_NAME, MINT_ENTRY_POINT_NAME, OWNER, RECIPIENT, SPENDER, EVENTS_MODE},
    utils::{get_package_hash, read_from}, modalities::EventsMode,
};

use casper_event_standard::{Event, emit};

pub const PREFIX_HASH_KEY_NAME: &str = "cep18_package";
pub const EVENT_TYPE: &str = "event_type";
pub const TOKEN_AMOUNT: &str = "token_amount";
pub const LEN: &str = "len";
pub const EVENTS: &str = "events";

pub fn record_event_dictionary(event: Event) {
    let events_mode: EventsMode =
    EventsMode::try_from(read_from::<u8>(EVENTS_MODE)).unwrap_or_revert();

    match events_mode {
        EventsMode::NoEvents => {}
        EventsMode::CES => ces(event),
        EventsMode::CEP47 => cep47(event),
    }
}

pub enum Event {
    Mint(Mint),
    Burn(Burn),
    SetAllowance(SetAllowance),
    IncreaseAllowance(IncreaseAllowance),
    DecreaseAllowance(DecreaseAllowance),
    Transfer(Transfer),
    TransferFrom(TransferFrom),
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Mint {
    pub recipient: Key,
    pub amount: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Burn {
    pub owner: Key,
    pub amount: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SetAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct IncreaseAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
    pub inc_by: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct DecreaseAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
    pub decr_by: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub sender: Key,
    pub recipient: Key,
    pub amount: U256,
} 

#[derive(Event, Debug, PartialEq, Eq)]
pub struct TransferFrom {
    pub spender: Key,
    pub owner: Key,
    pub recipient: Key,
    pub amount: U256,
}

pub fn cep47(event: Event) {
    let package = get_package_hash().to_string();
    let event: BTreeMap<&str, String> = match event {
        Event::Mint(Mint { recipient, amount }) => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, MINT_ENTRY_POINT_NAME.to_string());
            event.insert(RECIPIENT, recipient.to_string());
            event.insert(TOKEN_AMOUNT, amount.to_string());
            event
        }
        Event::Burn(Burn { owner, amount }) => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, BURN_ENTRY_POINT_NAME.to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(TOKEN_AMOUNT, amount.to_string());
            event
        }
        Event::SetAllowance(SetAllowance {
            owner,
            spender,
            allowance,
        }) => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "SetAllowance".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(SPENDER, spender.to_string());
            event.insert(TOKEN_AMOUNT, allowance.to_string());
            event
        }
        Event::IncreaseAllowance(IncreaseAllowance {
            owner,
            spender,
            inc_by,
            allowance,
        }) => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "IncreaseAllowance".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(SPENDER, spender.to_string());
            event.insert("inc_by", inc_by.to_string());
            event.insert(TOKEN_AMOUNT, allowance.to_string());
            event
        }
        Event::DecreaseAllowance(DecreaseAllowance {
            owner,
            spender,
            allowance,
            decr_by,
        }) => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "DecreaseAllowance".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(SPENDER, spender.to_string());
            event.insert("decr_by", decr_by.to_string());
            event.insert(TOKEN_AMOUNT, allowance.to_string());
            event
        }
        Event::Transfer(Transfer {
            sender,
            recipient,
            amount,
        }) => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "Transfer".to_string());
            event.insert(OWNER, sender.to_string());
            event.insert(RECIPIENT, recipient.to_string());
            event.insert(TOKEN_AMOUNT, amount.to_string());
            event
        }
        Event::TransferFrom(TransferFrom {
            spender,
            owner,
            recipient,
            amount,
        }) => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "TransferFrom".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(SPENDER, spender.to_string());
            event.insert(RECIPIENT, recipient.to_string());
            event.insert(TOKEN_AMOUNT, amount.to_string());
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

fn ces(event: Event){
    match event {
        Event::Mint(ev) => emit(ev),
        Event::Burn(ev) => emit(ev),
        Event::SetAllowance(ev) => emit(ev),
        Event::IncreaseAllowance(ev) => emit(ev),
        Event::DecreaseAllowance(ev) => emit(ev),
        Event::Transfer(ev) => emit(ev),
        Event::TransferFrom(ev) => emit(ev),
    }
}