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
    ApiError, CLTyped, Key, PackageHash, URef, U256,
};
use core::convert::TryInto;

/// Retrieves the immediate caller of the current contract execution context as a [`Key`].
///
/// This function abstracts over the different kinds of entities that may invoke a contract:
/// legacy accounts, legacy contract packages, or new entities.
///
/// # Behavior
///
/// * **ACCOUNT (legacy or new entity account)**   Returns a `Key::Account` wrapping the
///   `AccountHash`.
/// * **CONTRACT (legacy contract package)**   Returns a `Key::Hash` wrapping the
///   `ContractPackageHash`.
/// * **ENTITY (new entity)**   Returns a `Key::Hash` wrapping the `PackageHash`.
/// * **Other / unexpected kinds**   Reverts with [`Cep18Error::InvalidContext`].
pub fn get_immediate_caller() -> Key {
    const ACCOUNT: u8 = 0;
    const PACKAGE: u8 = 1;
    const CONTRACT_PACKAGE: u8 = 2;
    const ENTITY: u8 = 3;
    const CONTRACT: u8 = 4;

    let caller_info = casper_get_immediate_caller().unwrap_or_revert();

    let caller = match caller_info.kind() {
        // Legacy or new entity account returns AccountHash
        ACCOUNT => caller_info
            .get_field_by_index(ACCOUNT)
            .unwrap()
            .to_t::<Option<AccountHash>>()
            .unwrap_or_revert()
            .unwrap_or_revert_with(Cep18Error::InvalidContext)
            .into(),
        // New entity returns PackageHash
        ENTITY => caller_info
            .get_field_by_index(PACKAGE)
            .unwrap()
            .to_t::<Option<PackageHash>>()
            .unwrap_or_revert()
            .unwrap_or_revert_with(Cep18Error::InvalidContext)
            .into(),
        // Legacy returns ContractPackageHash
        CONTRACT => caller_info
            .get_field_by_index(CONTRACT_PACKAGE)
            .unwrap()
            .to_t::<Option<ContractPackageHash>>()
            .unwrap_or_revert()
            .unwrap_or_revert_with(Cep18Error::InvalidContext)
            .into(),
        _ => revert(Cep18Error::InvalidContext),
    };

    // Transform the caller Key to a legacy-compatible form (Account or Hash) for consistent
    // on-chain usage.
    // ⚠️ Strongly recommended: apply `key_as_account_or_package()` to any `Key` retrieved from
    // named arguments before comparing with the caller. This ensures consistent normalization
    // between user input and immediate caller, preventing mismatches.
    key_as_account_or_package(caller)
}

/// Converts a new-style [`Key`] returned by [`get_immediate_caller`] into a legacy-compatible
/// [`Key`].
///
/// This function ensures backward compatibility with legacy CEP-18 expectations,
/// where callers were identified only as `Account` or `Hash`.
///
/// # Behavior
///
/// * **`Key::AddressableEntity`**
///   - If the entity is an **account**, returns a legacy [`Key::Account`].
///   - If the entity is not an account (e.g., smart contract or system entity), returns a legacy
///     [`Key::Hash`]. This case should not normally occur in CEP-18, where contracts calling are
///     expected to be identified as contract packages, but it is handled for consistency.
/// * **`Key::SmartContract`**   Converts directly into a legacy [`Key::Hash`] using the
///   [`PackageHash`].
/// * **Other legacy keys** (`Key::Account`, `Key::Hash`)   Returned unchanged.
///
/// # Notes
///
/// - This function is mostly used in CEP-18 context where contract logic needs to interact with
///   storage or access controls (balances, allowances, etc.).
pub fn key_as_account_or_package(key: Key) -> Key {
    match key {
        Key::AddressableEntity(entity_addr) => {
            if entity_addr.is_account() {
                let account_hash = AccountHash::new(entity_addr.value());
                Key::Account(account_hash)
            } else {
                // This case should in theory never happen, since caller returned by
                // `get_immediate_caller` is expected to be either an Account or a
                // Package. We keep consistency by returning a legacy Key::Hash
                // instead of reverting here.
                Key::Hash(entity_addr.value())
            }
        }
        // Manage PackageHash from `get_immediate_caller` ENTITY case
        Key::SmartContract(package_addr) => Key::Hash(package_addr),
        // Legacy cases Account + ContractPackageHash from `get_immediate_caller` ACCOUNT + CONTRACT
        // cases
        legacy => legacy,
    }
}

pub trait UpsertTransform: Sized {
    fn key_as_account_or_package(self) -> Self {
        self
    }
}

impl UpsertTransform for Key {
    fn key_as_account_or_package(self) -> Self {
        key_as_account_or_package(self)
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

/// Creates a dictionary item key for a dictionary item, by base64 encoding the Key argument
/// since stringified Keys are too long to be used as dictionary keys.
#[inline]
pub fn make_dictionary_item_key(owner: Key) -> String {
    let preimage = owner
        .to_bytes()
        .unwrap_or_revert_with(Cep18Error::FailedToConvertBytes);
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is 33 bytes for
    // both used Key variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    base64_encode(preimage)
}

#[inline]
pub fn make_dictionary_item_key_value<T: CLTyped + ToBytes, V: CLTyped + ToBytes>(
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
