use crate::security::SecurityBadge;
#[cfg(feature = "contract-support")]
use crate::utils::get_stored_value;
#[cfg(feature = "contract-support")]
use crate::{
    constants::{ARG_EVENTS, ARG_EVENTS_MODE},
    error::Cep18Error,
    modalities::EventsMode,
};
use alloc::collections::BTreeMap;
#[cfg(feature = "contract-support")]
use alloc::string::String;
#[cfg(feature = "contract-support")]
use casper_contract::{
    contract_api::runtime::{emit_message, get_key},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_event_standard::Event;
#[cfg(feature = "contract-support")]
use casper_event_standard::{emit, init, Schemas, EVENTS_DICT};
#[cfg(feature = "contract-support")]
use casper_types::{bytesrepr::Bytes, contract_messages::MessagePayload};
use casper_types::{Key, U256};
use serde::{Deserialize, Serialize};

#[cfg(feature = "contract-support")]
pub fn record_event_dictionary(event: Event) {
    let events_mode: EventsMode = EventsMode::try_from(get_stored_value::<u8>(ARG_EVENTS_MODE))
        .unwrap_or_revert_with(Cep18Error::InvalidEventsMode);

    match events_mode {
        EventsMode::NoEvents => {}
        EventsMode::CES => ces(event),
        EventsMode::Native => emit_message(ARG_EVENTS, &event.to_json().into()).unwrap_or_revert(),
        EventsMode::NativeBytes => {
            let payload = MessagePayload::Bytes(Bytes::from(event.to_json().as_bytes()));
            emit_message(ARG_EVENTS, &payload).unwrap_or_revert()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Event {
    Mint(Mint),
    Burn(Burn),
    SetAllowance(SetAllowance),
    IncreaseAllowance(IncreaseAllowance),
    DecreaseAllowance(DecreaseAllowance),
    Transfer(Transfer),
    TransferFrom(TransferFrom),
    ChangeSecurity(ChangeSecurity),
    ChangeEventsMode(ChangeEventsMode),
}

impl Event {
    #[cfg(feature = "contract-support")]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .map_err(|_| Cep18Error::FailedToConvertToJson)
            .unwrap_or_revert()
    }
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct Mint {
    pub recipient: Key,
    pub amount: U256,
}

impl Mint {
    pub fn new(recipient: Key, amount: U256) -> Self {
        Self { recipient, amount }
    }
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct Burn {
    pub owner: Key,
    pub amount: U256,
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct SetAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct IncreaseAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
    pub inc_by: U256,
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct DecreaseAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
    pub decr_by: U256,
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub sender: Key,
    pub recipient: Key,
    pub amount: U256,
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct TransferFrom {
    pub spender: Key,
    pub owner: Key,
    pub recipient: Key,
    pub amount: U256,
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct ChangeSecurity {
    pub admin: Key,
    pub sec_change_map: BTreeMap<Key, SecurityBadge>,
}

#[derive(Serialize, Deserialize, Event, Debug, PartialEq, Eq)]
pub struct ChangeEventsMode {
    pub events_mode: u8,
}

#[cfg(feature = "contract-support")]
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
        Event::ChangeEventsMode(ev) => emit(ev),
    }
}

#[cfg(feature = "contract-support")]
pub fn init_events() {
    let events_mode: EventsMode = EventsMode::try_from(get_stored_value::<u8>(ARG_EVENTS_MODE))
        .unwrap_or_revert_with(Cep18Error::InvalidEventsMode);

    if EventsMode::CES == events_mode && get_key(EVENTS_DICT).is_none() {
        let schemas = Schemas::new()
            .with::<Mint>()
            .with::<Burn>()
            .with::<SetAllowance>()
            .with::<IncreaseAllowance>()
            .with::<DecreaseAllowance>()
            .with::<Transfer>()
            .with::<TransferFrom>()
            .with::<ChangeSecurity>()
            .with::<ChangeEventsMode>();
        init(schemas);
    }
}
