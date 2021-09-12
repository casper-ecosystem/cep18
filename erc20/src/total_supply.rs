//! Implementation of total supply.

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef, U256};

use crate::{constants::TOTAL_SUPPLY_KEY_NAME, detail};

#[inline]
pub(crate) fn total_supply_uref() -> URef {
    detail::get_uref(TOTAL_SUPPLY_KEY_NAME)
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn read_total_supply_from(uref: URef) -> U256 {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_total_supply_to(uref: URef, value: U256) {
    storage::write(uref, value);
}
