#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, call_contract, get_key, get_named_arg, put_key, revert},
        storage::{self, dictionary_put, read},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, contract_messages::MessageTopicOperation, contracts::ContractPackageHash,
    runtime_args, AddressableEntityHash, CLValue, EntityAddr, Key, NamedKeys, U256,
};
use cep18::{
    allowances::{read_allowance_from, write_allowance_to},
    balances::{read_balance_from, transfer_balance, write_balance_to},
    constants::{
        ADMIN_LIST, ARG_ADDRESS, ARG_AMOUNT, ARG_CONTRACT_HASH, ARG_DECIMALS, ARG_ENABLE_MINT_BURN,
        ARG_EVENTS, ARG_EVENTS_MODE, ARG_NAME, ARG_OWNER, ARG_PACKAGE_HASH, ARG_RECIPIENT,
        ARG_SPENDER, ARG_SYMBOL, ARG_TOTAL_SUPPLY, DICT_ALLOWANCES, DICT_BALANCES,
        DICT_SECURITY_BADGES, ENTRY_POINT_CHANGE_EVENTS_MODE, ENTRY_POINT_INIT, MINTER_LIST,
        NONE_LIST, PREFIX_ACCESS_KEY_NAME, PREFIX_CEP18, PREFIX_CONTRACT_NAME,
        PREFIX_CONTRACT_PACKAGE_NAME, PREFIX_CONTRACT_VERSION,
    },
    entry_points::generate_entry_points,
    error::Cep18Error,
    events::{
        self, init_events, Burn, ChangeEventsMode, ChangeSecurity, DecreaseAllowance, Event,
        IncreaseAllowance, Mint, SetAllowance, Transfer, TransferFrom,
    },
    modalities::EventsMode,
    security::{change_sec_badge, sec_check, SecurityBadge},
    utils::{
        base64_encode, get_contract_version_key, get_immediate_caller,
        get_optional_named_arg_with_user_errors, get_stored_value, get_uref_with_user_errors,
        write_total_supply_to,
    },
};

#[no_mangle]
pub extern "C" fn name() {
    runtime::ret(
        CLValue::from_t(get_stored_value::<String>(ARG_NAME))
            .unwrap_or_revert_with(Cep18Error::FailedToReturnEntryPointResult),
    );
}

#[no_mangle]
pub extern "C" fn symbol() {
    runtime::ret(
        CLValue::from_t(get_stored_value::<String>(ARG_SYMBOL))
            .unwrap_or_revert_with(Cep18Error::FailedToReturnEntryPointResult),
    );
}

#[no_mangle]
pub extern "C" fn decimals() {
    runtime::ret(
        CLValue::from_t(get_stored_value::<u8>(ARG_DECIMALS))
            .unwrap_or_revert_with(Cep18Error::FailedToReturnEntryPointResult),
    );
}

#[no_mangle]
pub extern "C" fn total_supply() {
    runtime::ret(
        CLValue::from_t(get_stored_value::<U256>(ARG_TOTAL_SUPPLY))
            .unwrap_or_revert_with(Cep18Error::FailedToReturnEntryPointResult),
    );
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Key = runtime::get_named_arg(ARG_ADDRESS);
    let balance = read_balance_from(address);
    runtime::ret(
        CLValue::from_t(balance).unwrap_or_revert_with(Cep18Error::FailedToReturnEntryPointResult),
    );
}

#[no_mangle]
pub extern "C" fn allowance() {
    let spender: Key = runtime::get_named_arg(ARG_SPENDER);
    let owner: Key = runtime::get_named_arg(ARG_OWNER);
    let val: U256 = read_allowance_from(owner, spender);
    runtime::ret(
        CLValue::from_t(val).unwrap_or_revert_with(Cep18Error::FailedToReturnEntryPointResult),
    );
}

#[no_mangle]
pub extern "C" fn approve() {
    let caller = get_immediate_caller();
    let spender: Key = runtime::get_named_arg(ARG_SPENDER);
    if spender == caller {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);
    write_allowance_to(caller, spender, amount);
    events::record_event_dictionary(Event::SetAllowance(SetAllowance {
        owner: caller,
        spender,
        allowance: amount,
    }))
}

#[no_mangle]
pub extern "C" fn decrease_allowance() {
    let caller = get_immediate_caller();
    let spender: Key = runtime::get_named_arg(ARG_SPENDER);
    if spender == caller {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);
    let current_allowance = read_allowance_from(caller, spender);
    let new_allowance = current_allowance.saturating_sub(amount);
    write_allowance_to(caller, spender, new_allowance);
    events::record_event_dictionary(Event::DecreaseAllowance(DecreaseAllowance {
        owner: caller,
        spender,
        decr_by: amount,
        allowance: new_allowance,
    }))
}

#[no_mangle]
pub extern "C" fn increase_allowance() {
    let caller = get_immediate_caller();
    let spender: Key = runtime::get_named_arg(ARG_SPENDER);
    if spender == caller {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);
    let current_allowance = read_allowance_from(caller, spender);
    let new_allowance = current_allowance.saturating_add(amount);
    write_allowance_to(caller, spender, new_allowance);
    events::record_event_dictionary(Event::IncreaseAllowance(IncreaseAllowance {
        owner: caller,
        spender,
        allowance: new_allowance,
        inc_by: amount,
    }))
}

#[no_mangle]
pub extern "C" fn transfer() {
    let caller = get_immediate_caller();
    let recipient: Key = runtime::get_named_arg(ARG_RECIPIENT);
    if caller == recipient {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);
    transfer_balance(caller, recipient, amount).unwrap_or_revert();
    events::record_event_dictionary(Event::Transfer(Transfer {
        sender: caller,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let caller = get_immediate_caller();
    let recipient: Key = runtime::get_named_arg(ARG_RECIPIENT);
    let owner: Key = runtime::get_named_arg(ARG_OWNER);
    if owner == recipient {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);
    if amount.is_zero() {
        return;
    }

    let spender_allowance: U256 = read_allowance_from(owner, caller);
    let new_spender_allowance = spender_allowance
        .checked_sub(amount)
        .unwrap_or_revert_with(Cep18Error::InsufficientAllowance);

    transfer_balance(owner, recipient, amount).unwrap_or_revert();
    write_allowance_to(owner, caller, new_spender_allowance);
    events::record_event_dictionary(Event::TransferFrom(TransferFrom {
        spender: caller,
        owner,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn mint() {
    if 0 == get_stored_value::<u8>(ARG_ENABLE_MINT_BURN) {
        revert(Cep18Error::MintBurnDisabled);
    }

    sec_check(vec![SecurityBadge::Admin, SecurityBadge::Minter]);

    let owner: Key = runtime::get_named_arg(ARG_OWNER);
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);

    let new_balance = {
        let balance = read_balance_from(owner);
        balance
            .checked_add(amount)
            .unwrap_or_revert_with(Cep18Error::Overflow)
    };
    let new_total_supply = {
        let total_supply: U256 = get_stored_value(ARG_TOTAL_SUPPLY);
        total_supply
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };

    write_balance_to(owner, new_balance);
    write_total_supply_to(new_total_supply);

    events::record_event_dictionary(Event::Mint(Mint {
        recipient: owner,
        amount,
    }));
}

#[no_mangle]
pub extern "C" fn burn() {
    if 0 == get_stored_value::<u8>(ARG_ENABLE_MINT_BURN) {
        revert(Cep18Error::MintBurnDisabled);
    }

    let owner: Key = runtime::get_named_arg(ARG_OWNER);
    let caller = get_immediate_caller();
    if owner != caller {
        revert(Cep18Error::InvalidBurnTarget);
    }

    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);
    let new_balance = {
        let balance = read_balance_from(owner);
        balance
            .checked_sub(amount)
            .unwrap_or_revert_with(Cep18Error::InsufficientBalance)
    };
    let new_total_supply = {
        let total_supply: U256 = get_stored_value(ARG_TOTAL_SUPPLY);
        total_supply
            .checked_sub(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert_with(Cep18Error::FailedToChangeTotalSupply)
    };

    write_balance_to(owner, new_balance);
    write_total_supply_to(new_total_supply);

    events::record_event_dictionary(Event::Burn(Burn { owner, amount }))
}

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    if get_key(DICT_ALLOWANCES).is_some() {
        revert(Cep18Error::AlreadyInitialized);
    }
    let package_hash = get_named_arg::<Key>(ARG_PACKAGE_HASH);
    put_key(ARG_PACKAGE_HASH, package_hash);

    let contract_hash = get_named_arg::<Key>(ARG_CONTRACT_HASH);
    put_key(ARG_CONTRACT_HASH, contract_hash);

    storage::new_dictionary(DICT_ALLOWANCES)
        .unwrap_or_revert_with(Cep18Error::FailedToCreateDictionary);
    storage::new_dictionary(DICT_BALANCES)
        .unwrap_or_revert_with(Cep18Error::FailedToCreateDictionary);
    let initial_supply = runtime::get_named_arg(ARG_TOTAL_SUPPLY);

    let caller = get_immediate_caller();

    let initial_balance_holder_key = caller;

    write_balance_to(initial_balance_holder_key, initial_supply);

    let security_badges_dict = storage::new_dictionary(DICT_SECURITY_BADGES)
        .unwrap_or_revert_with(Cep18Error::FailedToCreateDictionary);
    dictionary_put(
        security_badges_dict,
        &base64_encode(
            initial_balance_holder_key
                .to_bytes()
                .unwrap_or_revert_with(Cep18Error::FailedToConvertBytes),
        ),
        SecurityBadge::Admin,
    );

    let admin_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep18Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(MINTER_LIST, Cep18Error::InvalidMinterList);

    init_events();

    if let Some(minter_list) = minter_list {
        for minter in minter_list {
            dictionary_put(
                security_badges_dict,
                &base64_encode(
                    minter
                        .to_bytes()
                        .unwrap_or_revert_with(Cep18Error::FailedToConvertBytes),
                ),
                SecurityBadge::Minter,
            );
        }
    }
    if let Some(admin_list) = admin_list {
        for admin in admin_list {
            dictionary_put(
                security_badges_dict,
                &base64_encode(
                    admin
                        .to_bytes()
                        .unwrap_or_revert_with(Cep18Error::FailedToConvertBytes),
                ),
                SecurityBadge::Admin,
            );
        }
    }

    events::record_event_dictionary(Event::Mint(Mint {
        recipient: initial_balance_holder_key,
        amount: initial_supply,
    }));
}

/// Admin EntryPoint to manipulate the security access granted to users.
/// One user can only possess one access group badge.
/// Change strength: None > Admin > Minter
/// Change strength meaning by example: If user is added to both Minter and Admin they will be an
/// Admin, also if a user is added to Admin and None then they will be removed from having rights.
/// Beware: do not remove the last Admin because that will lock out all admin functionality.
#[no_mangle]
pub extern "C" fn change_security() {
    if 0 == get_stored_value::<u8>(ARG_ENABLE_MINT_BURN) {
        revert(Cep18Error::MintBurnDisabled);
    }
    sec_check(vec![SecurityBadge::Admin]);
    let admin_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep18Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(MINTER_LIST, Cep18Error::InvalidMinterList);
    let none_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(NONE_LIST, Cep18Error::InvalidNoneList);

    let mut badge_map: BTreeMap<Key, SecurityBadge> = BTreeMap::new();
    if let Some(minter_list) = minter_list {
        for account_key in minter_list {
            badge_map.insert(account_key, SecurityBadge::Minter);
        }
    }
    if let Some(admin_list) = admin_list {
        for account_key in admin_list {
            badge_map.insert(account_key, SecurityBadge::Admin);
        }
    }
    if let Some(none_list) = none_list {
        for account_key in none_list {
            badge_map.insert(account_key, SecurityBadge::None);
        }
    }

    let caller = get_immediate_caller();
    badge_map.remove(&caller);

    change_sec_badge(&badge_map);
    events::record_event_dictionary(Event::ChangeSecurity(ChangeSecurity {
        admin: caller,
        sec_change_map: badge_map,
    }));
}

#[no_mangle]
fn change_events_mode() {
    sec_check(vec![SecurityBadge::Admin]);
    let events_mode: EventsMode = EventsMode::try_from(get_named_arg::<u8>(ARG_EVENTS_MODE))
        .unwrap_or_revert_with(Cep18Error::InvalidEventsMode);
    let old_events_mode: EventsMode = EventsMode::try_from(get_stored_value::<u8>(ARG_EVENTS_MODE))
        .unwrap_or_revert_with(Cep18Error::InvalidEventsMode);
    if events_mode == old_events_mode {
        return;
    }
    let events_mode_u8 = events_mode as u8;
    put_key(ARG_EVENTS_MODE, storage::new_uref(events_mode_u8).into());
    init_events();

    events::record_event_dictionary(Event::ChangeEventsMode(ChangeEventsMode {
        events_mode: events_mode_u8,
    }));
}

pub fn upgrade(name: &str) {
    let entry_points = generate_entry_points();

    let package_key_name = &format!("{PREFIX_CEP18}_{PREFIX_CONTRACT_PACKAGE_NAME}_{name}");
    let contract_key_name = &format!("{PREFIX_CEP18}_{PREFIX_CONTRACT_NAME}_{name}");

    let old_contract_package_hash = match runtime::get_key(package_key_name)
        .unwrap_or_revert_with(Cep18Error::FailedToGetOldPackageKey)
    {
        Key::Hash(contract_hash) => contract_hash,
        Key::AddressableEntity(EntityAddr::SmartContract(contract_hash)) => contract_hash,
        Key::SmartContract(package_hash) => package_hash,
        _ => revert(Cep18Error::MissingPackageHashForUpgrade),
    };
    let contract_package_hash = ContractPackageHash::new(old_contract_package_hash);

    let previous_contract_hash = match runtime::get_key(contract_key_name)
        .unwrap_or_revert_with(Cep18Error::FailedToGetOldContractHashKey)
    {
        Key::Hash(contract_hash) => contract_hash,
        Key::AddressableEntity(EntityAddr::SmartContract(contract_hash)) => contract_hash,
        _ => revert(Cep18Error::MissingContractHashForUpgrade),
    };
    let converted_previous_contract_hash = AddressableEntityHash::new(previous_contract_hash);

    let events_mode = get_optional_named_arg_with_user_errors::<u8>(
        ARG_EVENTS_MODE,
        Cep18Error::InvalidEventsMode,
    );

    let version_value_uref = get_uref_with_user_errors(
        &format!("{PREFIX_CEP18}_{PREFIX_CONTRACT_VERSION}_{name}"),
        Cep18Error::MissingVersionContractKey,
        Cep18Error::InvalidVersionContractKey,
    );

    let version_value: String = read(version_value_uref)
        .unwrap_or_default()
        .unwrap_or_default();

    // If stored version is a non empty string (and not a u32), it means it is already a 2.0
    // version, do not add message topics then, as already set when installed
    let message_topics: BTreeMap<String, MessageTopicOperation> = if !version_value.is_empty() {
        BTreeMap::new()
    } else {
        BTreeMap::from([(ARG_EVENTS.to_string(), MessageTopicOperation::Add)])
    };

    let (contract_hash, contract_version) = storage::add_contract_version(
        contract_package_hash,
        entry_points,
        NamedKeys::new(),
        message_topics,
    );

    storage::disable_contract_version(
        contract_package_hash,
        converted_previous_contract_hash.into(),
    )
    .unwrap_or_revert_with(Cep18Error::FailedToDisableContractVersion);

    // ! TODO Check why new_contract still using ContractPackageHash
    // migrate old ContractPackageHash as PackageHash so it's stored in a uniform format with the
    // `add_contract_version` implementation
    // runtime::put_key(package_key_name, contract_package_hash.into());

    runtime::put_key(
        &format!("{PREFIX_CEP18}_{PREFIX_CONTRACT_NAME}_{name}"),
        Key::Hash(contract_hash.value()),
    );

    runtime::put_key(
        &format!("{PREFIX_CEP18}_{PREFIX_CONTRACT_VERSION}_{name}"),
        storage::new_uref(get_contract_version_key(contract_version).to_string()).into(),
    );

    if let Some(events_mode_u8) = events_mode {
        call_contract::<()>(
            contract_hash,
            ENTRY_POINT_CHANGE_EVENTS_MODE,
            runtime_args! {
                ARG_EVENTS_MODE => events_mode_u8,
            },
        );
    }
}

pub fn install_contract(name: &str) {
    let symbol: String = runtime::get_named_arg(ARG_SYMBOL);
    let decimals: u8 = runtime::get_named_arg(ARG_DECIMALS);
    let total_supply: U256 = runtime::get_named_arg(ARG_TOTAL_SUPPLY);
    let events_mode: u8 =
        get_optional_named_arg_with_user_errors(ARG_EVENTS_MODE, Cep18Error::InvalidEventsMode)
            .unwrap_or(0u8);

    let admin_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep18Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(MINTER_LIST, Cep18Error::InvalidMinterList);

    let enable_mint_burn: u8 = get_optional_named_arg_with_user_errors(
        ARG_ENABLE_MINT_BURN,
        Cep18Error::InvalidEnableMBFlag,
    )
    .unwrap_or(0);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(ARG_NAME.to_string(), storage::new_uref(name).into());
    named_keys.insert(ARG_SYMBOL.to_string(), storage::new_uref(symbol).into());
    named_keys.insert(ARG_DECIMALS.to_string(), storage::new_uref(decimals).into());
    named_keys.insert(
        ARG_TOTAL_SUPPLY.to_string(),
        storage::new_uref(total_supply).into(),
    );
    named_keys.insert(
        ARG_EVENTS_MODE.to_string(),
        storage::new_uref(events_mode).into(),
    );
    named_keys.insert(
        ARG_ENABLE_MINT_BURN.to_string(),
        storage::new_uref(enable_mint_burn).into(),
    );

    let entry_points = generate_entry_points();

    let message_topics = BTreeMap::from([(ARG_EVENTS.to_string(), MessageTopicOperation::Add)]);

    let package_hash_name = format!("{PREFIX_CEP18}_{PREFIX_CONTRACT_PACKAGE_NAME}_{name}");

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_hash_name.clone()),
        Some(format!("{PREFIX_CEP18}_{PREFIX_ACCESS_KEY_NAME}_{name}")),
        Some(message_topics),
    );

    let package_hash = runtime::get_key(&package_hash_name)
        .unwrap_or_revert_with(Cep18Error::FailedToGetPackageKey);

    let contract_hash_key = Key::Hash(contract_hash.value());

    // Store contract_hash and contract_version under account keys
    runtime::put_key(
        &format!("{PREFIX_CEP18}_{PREFIX_CONTRACT_NAME}_{name}"),
        contract_hash_key,
    );

    runtime::put_key(
        &format!("{PREFIX_CEP18}_{PREFIX_CONTRACT_VERSION}_{name}"),
        storage::new_uref(get_contract_version_key(contract_version).to_string()).into(),
    );

    // Call contract to initialize it
    let mut init_args = runtime_args! {ARG_TOTAL_SUPPLY => total_supply, ARG_PACKAGE_HASH => package_hash, ARG_CONTRACT_HASH => contract_hash_key, ARG_EVENTS_MODE => events_mode};

    if let Some(admin_list) = admin_list {
        init_args
            .insert(ADMIN_LIST, admin_list)
            .unwrap_or_revert_with(Cep18Error::FailedToInsertToSecurityList);
    }
    if let Some(minter_list) = minter_list {
        init_args
            .insert(MINTER_LIST, minter_list)
            .unwrap_or_revert_with(Cep18Error::FailedToInsertToSecurityList);
    }

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, init_args);
}

#[no_mangle]
pub extern "C" fn call() {
    let name: String = runtime::get_named_arg(ARG_NAME);
    match runtime::get_key(&format!("{PREFIX_CEP18}_{PREFIX_ACCESS_KEY_NAME}_{name}")) {
        Some(_) => {
            upgrade(&name);
        }
        None => {
            install_contract(&name);
        }
    }
}
