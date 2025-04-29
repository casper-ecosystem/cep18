#[cfg(feature = "contract-support")]
use crate::{
    constants::DICT_SECURITY_BADGES,
    error::Cep18Error,
    utils::{base64_encode, get_immediate_caller},
};
#[cfg(feature = "contract-support")]
use alloc::collections::BTreeMap;
use alloc::{vec, vec::Vec};
#[cfg(feature = "contract-support")]
use casper_contract::{contract_api::runtime::revert, unwrap_or_revert::UnwrapOrRevert};
#[cfg(feature = "contract-support")]
use casper_types::Key;
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLTyped,
};
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SecurityBadge {
    Admin = 0,
    Minter = 1,
    None = 2,
}

impl CLTyped for SecurityBadge {
    fn cl_type() -> casper_types::CLType {
        casper_types::CLType::U8
    }
}

impl ToBytes for SecurityBadge {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        Ok(vec![*self as u8])
    }

    fn serialized_length(&self) -> usize {
        1
    }
}

impl FromBytes for SecurityBadge {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        Ok((
            match bytes[0] {
                0 => SecurityBadge::Admin,
                1 => SecurityBadge::Minter,
                2 => SecurityBadge::None,
                _ => return Err(bytesrepr::Error::LeftOverBytes),
            },
            &[],
        ))
    }
}

#[cfg(feature = "contract-support")]
pub fn sec_check(allowed_badge_list: Vec<SecurityBadge>) {
    use crate::utils::get_dictionary_value_from_key;

    let caller = get_immediate_caller();
    let caller = caller.to_bytes().unwrap_or_revert();

    let badge: SecurityBadge =
        get_dictionary_value_from_key(DICT_SECURITY_BADGES, &base64_encode(caller))
            .unwrap_or_revert_with(Cep18Error::InsufficientRights);

    if !allowed_badge_list.contains(&badge) {
        revert(Cep18Error::InsufficientRights)
    }
}

#[cfg(feature = "contract-support")]
pub fn change_sec_badge(badge_map: &BTreeMap<Key, SecurityBadge>) {
    use crate::utils::set_dictionary_value_for_key;

    for (&user, &badge) in badge_map {
        set_dictionary_value_for_key(
            DICT_SECURITY_BADGES,
            &base64_encode(user.to_bytes().unwrap_or_revert()),
            &badge,
        )
    }
}
