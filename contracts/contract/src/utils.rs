use crate::{constants::ARG_TOTAL_SUPPLY, error::Cep18Error};
use alloc::{string::String, vec, vec::Vec};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use casper_contract::{
    contract_api::{
        self, alloc_bytes,
        runtime::{get_key, revert, PROTOCOL_VERSION_FIELD_IDX, PROTOCOL_VERSION_LENGTH},
        storage::{dictionary_get, dictionary_put, read, write},
    },
    ext_ffi::{self, casper_get_block_info},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error,
    bytesrepr::{self, FromBytes, ToBytes},
    contracts::{ContractVersionKey, ProtocolVersionMajor},
    system::CallStackElement,
    ApiError, CLTyped, Key, URef, U256,
};
use core::{convert::TryInto, mem::MaybeUninit};

// TODO CHECK *runtime::get_call_stack()
/// ! TODO GR
fn read_host_buffer(size: usize) -> Result<Vec<u8>, ApiError> {
    let mut dest: Vec<u8> = if size == 0 {
        Vec::new()
    } else {
        let bytes_non_null_ptr = contract_api::alloc_bytes(size);
        unsafe { Vec::from_raw_parts(bytes_non_null_ptr.as_ptr(), size, size) }
    };
    read_host_buffer_into(&mut dest)?;
    Ok(dest)
}

// TODO CHECK *runtime::get_call_stack()
/// ! TODO GR
fn read_host_buffer_into(dest: &mut [u8]) -> Result<usize, ApiError> {
    let mut bytes_written = MaybeUninit::uninit();
    let ret = unsafe {
        ext_ffi::casper_read_host_buffer(dest.as_mut_ptr(), dest.len(), bytes_written.as_mut_ptr())
    };
    // NOTE: When rewriting below expression as `result_from(ret).map(|_| unsafe { ... })`, and the
    // caller ignores the return value, execution of the contract becomes unstable and ultimately
    // leads to `Unreachable` error.
    api_error::result_from(ret)?;
    Ok(unsafe { bytes_written.assume_init() })
}

// TODO CHECK *runtime::get_call_stack()
/// ! TODO GR
pub fn get_call_stack() -> Vec<CallStackElement> {
    let (call_stack_len, result_size) = {
        let mut call_stack_len: usize = 0;
        let mut result_size: usize = 0;
        let ret = unsafe {
            #[allow(deprecated)]
            ext_ffi::casper_load_call_stack(
                &mut call_stack_len as *mut usize,
                &mut result_size as *mut usize,
            )
        };
        api_error::result_from(ret).unwrap_or_revert();
        (call_stack_len, result_size)
    };
    if call_stack_len == 0 {
        return Vec::new();
    }
    let bytes = read_host_buffer(result_size).unwrap_or_revert();
    bytesrepr::deserialize(bytes).unwrap_or_revert()
}

// CHECK *runtime::get_call_stack() // get_immediate_caller() CallerInfo into Key (Caller ?)
/// ! TODO GR
pub fn get_immediate_caller() -> Key {
    match *get_call_stack().iter().nth_back(1).unwrap_or_revert() {
        CallStackElement::Session { account_hash } => Key::from(account_hash),
        CallStackElement::StoredSession {
            account_hash: _, // Caller is contract
            contract_package_hash,
            contract_hash: _,
        } => contract_package_hash.into(),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => contract_package_hash.into(),
    }
}

pub fn get_contract_version_key(contract_version: u32) -> ContractVersionKey {
    let dest_ptr = alloc_bytes(PROTOCOL_VERSION_LENGTH as usize);
    unsafe {
        casper_get_block_info(PROTOCOL_VERSION_FIELD_IDX, dest_ptr.as_ptr());
        let protocol_version_major = ProtocolVersionMajor::from(u32::from_ne_bytes(
            core::ptr::read_unaligned(dest_ptr.as_ptr() as *const [u8; 4]),
        ));
        ContractVersionKey::new(protocol_version_major, contract_version)
    }
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
                ext_ffi::casper_get_named_arg(
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

pub fn get_stored_value_with_user_errors<T: CLTyped + FromBytes>(
    name: &str,
    missing: Cep18Error,
    invalid: Cep18Error,
) -> T {
    let uref = get_uref_with_user_errors(name, missing, invalid);
    read_with_user_errors(uref, missing, invalid)
}

pub fn make_dictionary_item_key<T: CLTyped + ToBytes, V: CLTyped + ToBytes>(
    key: &T,
    value: &V,
) -> String {
    use casper_contract::contract_api::runtime::blake2b;

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

fn get_uref_with_user_errors(name: &str, missing: Cep18Error, invalid: Cep18Error) -> URef {
    let key = get_key_with_user_errors(name, missing, invalid);
    key.into_uref()
        .unwrap_or_revert_with(Cep18Error::InvalidKeyType)
}

fn get_key_with_user_errors(name: &str, missing: Cep18Error, invalid: Cep18Error) -> Key {
    let (name_ptr, name_size, _bytes) = to_ptr(name);
    let mut key_bytes = vec![0u8; Key::max_serialized_length()];
    let mut total_bytes: usize = 0;
    let ret = unsafe {
        ext_ffi::casper_get_key(
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

fn read_with_user_errors<T: CLTyped + FromBytes>(
    uref: URef,
    missing: Cep18Error,
    invalid: Cep18Error,
) -> T {
    let key: Key = uref.into();
    let (key_ptr, key_size, _bytes) = to_ptr(key);

    let value_size = {
        let mut value_size = MaybeUninit::uninit();
        let ret = unsafe { ext_ffi::casper_read_value(key_ptr, key_size, value_size.as_mut_ptr()) };
        match api_error::result_from(ret) {
            Ok(_) => unsafe { value_size.assume_init() },
            Err(ApiError::ValueNotFound) => revert(missing),
            Err(e) => revert(e),
        }
    };

    let value_bytes = read_host_buffer(value_size).unwrap_or_revert();

    bytesrepr::deserialize(value_bytes).unwrap_or_revert_with(invalid)
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
        ext_ffi::casper_get_named_arg_size(
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
