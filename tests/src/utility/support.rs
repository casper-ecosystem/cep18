use std::fmt::Debug;

use casper_engine_test_support::LmdbWasmTestBuilder;
use casper_event_standard::EVENTS_DICT;
use casper_types::{
    bytesrepr::{Bytes, FromBytes},
    AddressableEntityHash, CLTyped, EntityAddr, Key,
};

pub(crate) fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    builder: &LmdbWasmTestBuilder,
    contract_key: &Key,
    dictionary_name: &str,
    dictionary_key: &str,
) -> T {
    let named_key = match contract_key.into_entity_hash() {
        Some(hash) => {
            let entity_with_named_keys = builder
                .get_entity_with_named_keys_by_entity_hash(hash)
                .expect("should be named key from entity hash");
            let named_keys = entity_with_named_keys.named_keys();
            named_keys
                .get(dictionary_name)
                .expect("must have key")
                .to_owned()
        }
        None => match contract_key.into_hash_addr() {
            Some(contract_key) => {
                let named_keys = builder.get_named_keys(EntityAddr::SmartContract(contract_key));
                named_keys
                    .get(dictionary_name)
                    .expect("must have key")
                    .to_owned()
            }
            None => {
                panic!("unsupported dictionary location")
            }
        },
    };

    let seed_uref = named_key.as_uref().expect("must convert to seed uref");

    builder
        .query_dictionary_item(None, *seed_uref, dictionary_key)
        .expect("should have dictionary value")
        .as_cl_value()
        .expect("T should be CLValue")
        .to_owned()
        .into_t()
        .unwrap()
}

pub(crate) fn query_stored_value<T: CLTyped + FromBytes>(
    builder: &LmdbWasmTestBuilder,
    base_key: Key,
    name: &str,
) -> T {
    let stored = builder.query(None, base_key, &[name.to_string()]);
    let cl_value = stored
        .expect("must have stored value")
        .as_cl_value()
        .cloned()
        .expect("must have cl value");

    cl_value.into_t::<T>().expect("must get value")
}

pub fn get_event<T: FromBytes + CLTyped + Debug>(
    builder: &mut LmdbWasmTestBuilder,
    contract_hash: &AddressableEntityHash,
    index: u32,
) -> T {
    let bytes: Bytes = get_dictionary_value_from_key(
        builder,
        &Key::contract_entity_key(*contract_hash),
        EVENTS_DICT,
        &index.to_string(),
    );
    let (event, bytes) = T::from_bytes(&bytes).unwrap();
    assert!(bytes.is_empty());
    event
}
