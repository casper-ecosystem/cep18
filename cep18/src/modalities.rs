use core::convert::TryFrom;

use crate::Cep18Error;

#[repr(u8)]
#[derive(PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum EventsMode {
    NoEvents = 0,
    CEP47 = 1,
    CES = 2,
}

impl TryFrom<u8> for EventsMode {
    type Error = Cep18Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventsMode::NoEvents),
            1 => Ok(EventsMode::CEP47),
            2 => Ok(EventsMode::CES),
            _ => Err(Cep18Error::InvalidEventsMode),
        }
    }
}