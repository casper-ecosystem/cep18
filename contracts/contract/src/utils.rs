use crate::{constants::ARG_TOTAL_SUPPLY, error::Cep18Error};
use alloc::{string::String, vec, vec::Vec};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use casper_contract::{
    contract_api::{
        self,
        runtime::{
            blake2b, get_immediate_caller as casper_get_immediate_caller, get_key,
            get_protocol_version, revert,
        },
        storage::{dictionary_get, dictionary_put, read, write},
    },
    ext_ffi::{casper_get_key, casper_get_named_arg, casper_get_named_arg_size},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    api_error,
    bytesrepr::{self, FromBytes, ToBytes},
    contracts::{ContractPackageHash, ContractVersionKey},
    ApiError, CLTyped, EntityAddr, Key, URef, U256,
};
use core::convert::TryInto;

pub fn get_immediate_caller() -> Key {
    const ACCOUNT: u8 = 0;
    const CONTRACT_PACKAGE: u8 = 2;
    const ENTITY: u8 = 3;
    const CONTRACT: u8 = 4;

    let caller_info = casper_get_immediate_caller().unwrap_or_revert();

    match caller_info.kind() {
        ACCOUNT => caller_info
            .get_field_by_index(ACCOUNT)
            .unwrap()
            .to_t::<Option<AccountHash>>()
            .unwrap_or_revert()
            .unwrap_or_revert_with(Cep18Error::InvalidContext)
            .into(),
        CONTRACT => caller_info
            .get_field_by_index(CONTRACT_PACKAGE)
            .unwrap()
            .to_t::<Option<ContractPackageHash>>()
            .unwrap_or_revert()
            .unwrap_or_revert_with(Cep18Error::InvalidContext)
            .into(),
        ENTITY => caller_info
            .get_field_by_index(ENTITY)
            .unwrap()
            .to_t::<Option<EntityAddr>>()
            .unwrap_or_revert()
            .unwrap_or_revert_with(Cep18Error::InvalidContext)
            .into(),
        _ => revert(Cep18Error::InvalidContext),
    }
}

pub fn get_contract_version_key(contract_version: u32) -> ContractVersionKey {
    let (major, _, _) = get_protocol_version().destructure();
    ContractVersionKey::new(major, contract_version)
}

/// Reads value from a named key.
pub fn get_stored_value<T>(name: &str) -> T
where
    T: FromBytes + CLTyped,
{
    let uref = get_uref(name);
    read(uref)
        .unwrap_or_revert_with(Cep18Error::UrefNotFound)
        .unwrap_or_revert_with(Cep18Error::FailedToReadFromStorage)
}

// This method Unused for now in the contract but maybe usefull later
// pub fn get_stored_value_with_user_errors<T: CLTyped + FromBytes>(
//     name: &str,
//     missing: Cep18Error,
//     invalid: Cep18Error,
// ) -> T {
//     let uref = get_uref_with_user_errors(name, missing, invalid);
//     read::<T>(uref)
//         .unwrap_or_revert_with(missing)
//         .unwrap_or_revert_with(invalid)
// }

pub fn get_named_arg_with_user_errors<T: FromBytes>(
    name: &str,
    missing: Cep18Error,
    invalid: Cep18Error,
) -> Result<T, Cep18Error> {
    let arg_size = get_named_arg_size(name).ok_or(missing)?;
    let arg_bytes = if arg_size > 0 {
        let res = {
            let data_non_null_ptr = contract_api::alloc_bytes(arg_size);
            let ret = unsafe {
                casper_get_named_arg(
                    name.as_bytes().as_ptr(),
                    name.len(),
                    data_non_null_ptr.as_ptr(),
                    arg_size,
                )
            };
            let data =
                unsafe { Vec::from_raw_parts(data_non_null_ptr.as_ptr(), arg_size, arg_size) };
            api_error::result_from(ret).map(|_| data)
        };
        // Assumed to be safe as `get_named_arg_size` checks the argument already
        res.unwrap_or_revert_with(Cep18Error::FailedToGetArgBytes)
    } else {
        // Avoids allocation with 0 bytes and a call to get_named_arg
        Vec::new()
    };

    bytesrepr::deserialize(arg_bytes).map_err(|_| invalid)
}

pub fn get_optional_named_arg_with_user_errors<T: FromBytes>(
    name: &str,
    invalid: Cep18Error,
) -> Option<T> {
    match get_named_arg_with_user_errors::<T>(name, Cep18Error::Phantom, invalid) {
        Ok(val) => Some(val),
        Err(Cep18Error::Phantom) => None,
        Err(e) => revert(e),
    }
}

pub fn make_dictionary_item_key<T: CLTyped + ToBytes, V: CLTyped + ToBytes>(
    key: &T,
    value: &V,
) -> String {
    let mut bytes_a = key
        .to_bytes()
        .unwrap_or_revert_with(Cep18Error::FailedToConvertBytes);
    let mut bytes_b = value
        .to_bytes()
        .unwrap_or_revert_with(Cep18Error::FailedToConvertBytes);

    bytes_a.append(&mut bytes_b);

    let bytes = blake2b(bytes_a);
    hex::encode(bytes)
}

pub fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    dictionary_name: &str,
    key: &str,
) -> Option<T> {
    let seed_uref = get_uref_with_user_errors(
        dictionary_name,
        Cep18Error::MissingStorageUref,
        Cep18Error::InvalidStorageUref,
    );

    match dictionary_get::<T>(seed_uref, key) {
        Ok(maybe_value) => maybe_value,
        Err(error) => revert(error),
    }
}

pub fn set_dictionary_value_for_key<T: CLTyped + ToBytes + Copy>(
    dictionary_name: &str,
    key: &str,
    value: &T,
) {
    let seed_uref = get_uref_with_user_errors(
        dictionary_name,
        Cep18Error::MissingStorageUref,
        Cep18Error::InvalidStorageUref,
    );
    dictionary_put::<T>(seed_uref, key, *value)
}

/// Gets [`URef`] under a name.
fn get_uref(name: &str) -> URef {
    let key = get_key(name)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert_with(Cep18Error::FailedToGetKey);
    key.try_into()
        .unwrap_or_revert_with(Cep18Error::InvalidKeyType)
}

pub fn get_uref_with_user_errors(name: &str, missing: Cep18Error, invalid: Cep18Error) -> URef {
    let key = get_key_with_user_errors(name, missing, invalid);
    key.into_uref()
        .unwrap_or_revert_with(Cep18Error::InvalidKeyType)
}

fn get_key_with_user_errors(name: &str, missing: Cep18Error, invalid: Cep18Error) -> Key {
    let (name_ptr, name_size, _bytes) = to_ptr(name);
    let mut key_bytes = vec![0u8; Key::max_serialized_length()];
    let mut total_bytes: usize = 0;
    let ret = unsafe {
        casper_get_key(
            name_ptr,
            name_size,
            key_bytes.as_mut_ptr(),
            key_bytes.len(),
            &mut total_bytes as *mut usize,
        )
    };
    match api_error::result_from(ret) {
        Ok(_) => {}
        Err(ApiError::MissingKey) => revert(missing),
        Err(e) => revert(e),
    }
    key_bytes.truncate(total_bytes);

    bytesrepr::deserialize(key_bytes).unwrap_or_revert_with(invalid)
}

fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
    let bytes = t.into_bytes().unwrap_or_revert();
    let ptr = bytes.as_ptr();
    let size = bytes.len();
    (ptr, size, bytes)
}

pub fn get_named_arg_size(name: &str) -> Option<usize> {
    let mut arg_size: usize = 0;
    let ret = unsafe {
        casper_get_named_arg_size(
            name.as_bytes().as_ptr(),
            name.len(),
            &mut arg_size as *mut usize,
        )
    };
    match api_error::result_from(ret) {
        Ok(_) => Some(arg_size),
        Err(ApiError::MissingArgument) => None,
        Err(e) => revert(e),
    }
}

pub fn base64_encode(data: Vec<u8>) -> String {
    STANDARD.encode(data)
}

// Writes a total supply to a specific [`URef`].
pub fn write_total_supply_to(value: U256) {
    get_uref(ARG_TOTAL_SUPPLY);
    write(get_uref(ARG_TOTAL_SUPPLY), value);
}
