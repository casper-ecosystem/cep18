use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::Key;

use crate::{
    constants::{
        EVENTS, EVENT_TYPE, OPERATOR, OWNER, PREFIX_CEP78, PREFIX_HASH_KEY_NAME, RECIPIENT, SENDER,
        SPENDER, TOKEN_ID,
    },
    error::NFTCoreError,
    modalities::TokenIdentifier,
    utils,
};

pub enum Event {
    Mint {
        recipient: Key,
        token_id: TokenIdentifier,
    },
    Burn {
        owner: Key,
        token_id: TokenIdentifier,
    },
    ApprovalGranted {
        owner: Key,
        spender: Key,
        token_id: TokenIdentifier,
    },
    ApprovalRevoked {
        owner: Key,
        token_id: TokenIdentifier,
    },
    ApprovalForAll {
        owner: Key,
        operator: Key,
    },
    RevokedForAll {
        owner: Key,
        operator: Key,
    },
    Transfer {
        sender: Key,
        recipient: Key,
        token_id: TokenIdentifier,
    },
    MetadataUpdate {
        token_id: TokenIdentifier,
    },
    VariablesSet,
    Migrate,
}

pub fn record__event_dictionary(event: Event) {
    let collection_name: String = utils::get_stored_value_with_user_errors(
        crate::constants::COLLECTION_NAME,
        NFTCoreError::MissingCollectionName,
        NFTCoreError::InvalidCollectionName,
    );

    let package = utils::get_stored_value_with_user_errors::<String>(
        &format!("{PREFIX_CEP78}_{collection_name}"),
        NFTCoreError::MissingCep78PackageHash,
        NFTCoreError::InvalidCep78InvalidHash,
    );

    let event: BTreeMap<&str, String> = match event {
        Event::Mint {
            recipient,
            token_id,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "Mint".to_string());
            event.insert(RECIPIENT, recipient.to_string());
            event.insert(TOKEN_ID, token_id.to_string());
            event
        }
        Event::Burn { owner, token_id } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "Burn".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(TOKEN_ID, token_id.to_string());
            event
        }
        Event::ApprovalGranted {
            owner,
            spender,
            token_id,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "Approve".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(SPENDER, spender.to_string());
            event.insert(TOKEN_ID, token_id.to_string());
            event
        }
        Event::ApprovalRevoked { owner, token_id } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "ApprovalRevoked".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(TOKEN_ID, token_id.to_string());
            event
        }
        Event::ApprovalForAll { owner, operator } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "ApprovalForAll".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(OPERATOR, operator.to_string());
            event
        }
        Event::RevokedForAll { owner, operator } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "RevokedForAll".to_string());
            event.insert(OWNER, owner.to_string());
            event.insert(OPERATOR, operator.to_string());
            event
        }
        Event::Transfer {
            sender,
            recipient,
            token_id,
        } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "Transfer".to_string());
            event.insert(SENDER, sender.to_string());
            event.insert(RECIPIENT, recipient.to_string());
            event.insert(TOKEN_ID, token_id.to_string());
            event
        }
        Event::MetadataUpdate { token_id } => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "MetadataUpdate".to_string());
            event.insert(TOKEN_ID, token_id.to_string());
            event
        }
        Event::Migrate => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "Migration".to_string());
            event
        }
        Event::VariablesSet => {
            let mut event = BTreeMap::new();
            event.insert(PREFIX_HASH_KEY_NAME, package);
            event.insert(EVENT_TYPE, "VariablesSet".to_string());
            event
        }
    };
    let dictionary_uref = match runtime::get_key(EVENTS) {
        Some(dict_uref) => dict_uref.into_uref().unwrap_or_revert(),
        None => storage::new_dictionary(EVENTS).unwrap_or_revert(),
    };
    let len = storage::dictionary_get(dictionary_uref, "len")
        .unwrap_or_revert()
        .unwrap_or(0_u64);
    storage::dictionary_put(dictionary_uref, &len.to_string(), event);
    storage::dictionary_put(dictionary_uref, "len", len + 1);
}
