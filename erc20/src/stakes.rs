//! Implementation of stakes.
use alloc::{string::String, vec::Vec};

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::ToBytes, URef, U256};

use crate::{
    constants::REWARDS_KEY_NAME, constants::STAKERS_KEY_NAME, constants::STAKES_KEY_NAME, detail,
    Address,
};

#[inline]
pub(crate) fn stakers_uref() -> URef {
    detail::get_uref(STAKERS_KEY_NAME)
}

#[inline]
pub(crate) fn stakes_uref() -> URef {
    detail::get_uref(STAKES_KEY_NAME)
}

#[inline]
pub(crate) fn rewards_uref() -> URef {
    detail::get_uref(REWARDS_KEY_NAME)
}

#[inline]
fn make_dictionary_item_key(owner: Address) -> String {
    let preimage = owner.to_bytes().unwrap_or_revert();

    base64::encode(&preimage)
}

/// Reads an stake for a owner
pub(crate) fn read_stake_from(stakes_uref: URef, owner: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(owner);
    storage::dictionary_get(stakes_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}

/// Reads a reward for a owner
pub(crate) fn read_reward_from(rewards_uref: URef, owner: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(owner);
    storage::dictionary_get(rewards_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}

/// Reads stakers
pub(crate) fn read_stakers_from(stakers_uref: URef) -> Vec<Address> {
    storage::dictionary_get(stakers_uref, STAKERS_KEY_NAME)
        .unwrap_or_revert()
        .unwrap_or_default()
}

/// Add staker
pub(crate) fn add_staker(stakers_uref: URef, staker: Address) -> bool {
    let mut stakers: Vec<Address> = read_stakers_from(stakers_uref);

    if !stakers.contains(&staker) {
        stakers.push(staker);
        storage::dictionary_put(stakers_uref, STAKERS_KEY_NAME, stakers);
        true
    } else {
        false
    }
}

/// Writes an stake for owner for a specific amount.
pub(crate) fn write_reward_to(rewards_uref: URef, owner: Address, amount: U256) {
    let dictionary_item_key = make_dictionary_item_key(owner);
    storage::dictionary_put(rewards_uref, &dictionary_item_key, amount)
}
