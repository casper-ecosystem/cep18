use core::convert::TryFrom;

use alloc::collections::BTreeMap;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key, U256};

use crate::{
    constants::EVENTS_MODE,
    modalities::EventsMode,
    utils::{read_from, SecurityBadge},
};

use casper_event_standard::{emit, Event, Schemas};

pub fn record_event_dictionary(event: Event) {
    let events_mode: EventsMode =
        EventsMode::try_from(read_from::<u8>(EVENTS_MODE)).unwrap_or_revert();

    match events_mode {
        EventsMode::NoEvents => {}
        EventsMode::CES => ces(event),
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
    ChangeSecurity(ChangeSecurity),
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

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ChangeSecurity {
    pub admin: Key,
    pub sec_change_map: BTreeMap<Key, SecurityBadge>,
}

fn ces(event: Event) {
    match event {
        Event::Mint(ev) => emit(ev),
        Event::Burn(ev) => emit(ev),
        Event::SetAllowance(ev) => emit(ev),
        Event::IncreaseAllowance(ev) => emit(ev),
        Event::DecreaseAllowance(ev) => emit(ev),
        Event::Transfer(ev) => emit(ev),
        Event::TransferFrom(ev) => emit(ev),
        Event::ChangeSecurity(ev) => emit(ev),
    }
}

pub fn init_events() {
    let events_mode: EventsMode =
        EventsMode::try_from(read_from::<u8>(EVENTS_MODE)).unwrap_or_revert();

    if events_mode == EventsMode::CES {
        let schemas = Schemas::new()
            .with::<Mint>()
            .with::<Burn>()
            .with::<SetAllowance>()
            .with::<IncreaseAllowance>()
            .with::<DecreaseAllowance>()
            .with::<Transfer>()
            .with::<TransferFrom>()
            .with::<ChangeSecurity>();
        casper_event_standard::init(schemas);
    }
}
