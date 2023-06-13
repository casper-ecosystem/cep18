#![no_std]
#![no_main]

extern crate alloc;

mod allowances;
mod balances;
pub mod constants;
pub mod entry_points;
mod error;
mod events;
mod modalities;
mod utils;

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use allowances::{get_allowances_uref, read_allowance_from, write_allowance_to};
use balances::{get_balances_uref, read_balance_from, transfer_balance, write_balance_to};
use entry_points::{exchange_from_old_token as exchange_token, generate_entry_points};

use casper_contract::{
    contract_api::{
        runtime::{self, get_caller, get_key, get_named_arg, put_key, revert},
        storage::{self, dictionary_put},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, contracts::NamedKeys, runtime_args, CLValue, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};

use constants::{
    ACCESS_KEY_NAME_PREFIX, ADDRESS, ADMIN_LIST, ALLOWANCES, AMOUNT, BALANCES, BURNER_LIST,
    CONTRACT_NAME_PREFIX, CONTRACT_VERSION_PREFIX, DECIMALS, ENABLE_MINT_BURN, EVENTS_MODE,
    HASH_KEY_NAME_PREFIX, INIT_ENTRY_POINT_NAME, MINTER_LIST, MINT_AND_BURN_LIST, NAME, NONE_LIST,
    OWNER, PACKAGE_HASH, RECIPIENT, SECURITY_BADGES, SPENDER, SYMBOL, TOTAL_SUPPLY,
};
pub use error::Cep18Error;
use events::{
    init_events, Burn, ChangeSecurity, DecreaseAllowance, Event, IncreaseAllowance, Mint,
    SetAllowance, Transfer, TransferFrom,
};
use utils::{
    get_immediate_caller_address, get_total_supply_uref, read_from, read_total_supply_from,
    sec_check, write_total_supply_to, SecurityBadge,
};

#[no_mangle]
pub extern "C" fn name() {
    runtime::ret(CLValue::from_t(utils::read_from::<String>(NAME)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    runtime::ret(CLValue::from_t(utils::read_from::<String>(SYMBOL)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    runtime::ret(CLValue::from_t(utils::read_from::<u8>(DECIMALS)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    runtime::ret(CLValue::from_t(utils::read_from::<U256>(TOTAL_SUPPLY)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Key = runtime::get_named_arg(ADDRESS);
    let balances_uref = get_balances_uref();
    let balance = balances::read_balance_from(balances_uref, address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Key = runtime::get_named_arg(OWNER);
    let spender: Key = runtime::get_named_arg(SPENDER);
    let allowances_uref = get_allowances_uref();
    let val: U256 = read_allowance_from(allowances_uref, owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Key = runtime::get_named_arg(SPENDER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let allowances_uref = get_allowances_uref();
    write_allowance_to(allowances_uref, owner, spender, amount);
    events::record_event_dictionary(Event::SetAllowance(SetAllowance {
        owner,
        spender,
        allowance: amount,
    }))
}

#[no_mangle]
pub extern "C" fn decrease_allowance() {
    let spender: Key = runtime::get_named_arg(SPENDER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let allowances_uref = get_allowances_uref();
    let current_allowance = read_allowance_from(allowances_uref, owner, spender);
    let new_allowance = current_allowance.saturating_sub(amount);
    write_allowance_to(allowances_uref, owner, spender, new_allowance);
    events::record_event_dictionary(Event::DecreaseAllowance(DecreaseAllowance {
        owner,
        spender,
        decr_by: amount,
        allowance: new_allowance,
    }))
}

#[no_mangle]
pub extern "C" fn increase_allowance() {
    let spender: Key = runtime::get_named_arg(SPENDER);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let allowances_uref = get_allowances_uref();
    let current_allowance = read_allowance_from(allowances_uref, owner, spender);
    let new_allowance = current_allowance.saturating_add(amount);
    write_allowance_to(allowances_uref, owner, spender, new_allowance);
    events::record_event_dictionary(Event::IncreaseAllowance(IncreaseAllowance {
        owner,
        spender,
        allowance: new_allowance,
        inc_by: amount,
    }))
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Key = runtime::get_named_arg(RECIPIENT);
    let amount: U256 = runtime::get_named_arg(AMOUNT);

    let sender = utils::get_immediate_caller_address().unwrap_or_revert();

    transfer_balance(sender, recipient, amount).unwrap_or_revert();
    events::record_event_dictionary(Event::Transfer(Transfer {
        sender,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Key = runtime::get_named_arg(OWNER);
    let recipient: Key = runtime::get_named_arg(RECIPIENT);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let spender = utils::get_immediate_caller_address().unwrap_or_revert();
    if amount.is_zero() {
        return;
    }

    let allowances_uref = get_allowances_uref();
    let spender_allowance: U256 = read_allowance_from(allowances_uref, owner, spender);
    let new_spender_allowance = spender_allowance
        .checked_sub(amount)
        .ok_or(Cep18Error::InsufficientAllowance)
        .unwrap_or_revert();

    transfer_balance(owner, recipient, amount).unwrap_or_revert();
    write_allowance_to(allowances_uref, owner, spender, new_spender_allowance);
    events::record_event_dictionary(Event::TransferFrom(TransferFrom {
        spender,
        owner,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn mint() {
    if 0 == read_from::<u8>(ENABLE_MINT_BURN) {
        revert(Cep18Error::MintBurnDisabled);
    }
    // only minter can mint
    sec_check(vec![
        SecurityBadge::Admin,
        SecurityBadge::Minter,
        SecurityBadge::MintAndBurn,
    ]);

    let owner: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let mintid: String = runtime::get_named_arg("mintid");

    //check whether mintid is redeemed
    let mintid_key = utils::make_mintid_dictionary_item_key(&mintid);
    let mintid_value: u64 =
        utils::get_dictionary_value_from_key("mintids", &mintid_key).unwrap_or_default();
    if mintid_value > 0 {
        runtime::revert(Cep18Error::AlreadyMint);
    }
    //write mintid for redeem state
    utils::write_dictionary_value_from_key("mintids", &mintid_key, 1u64);

    //check swap fee
    let swap_fee: U256 = utils::get_key("swap_fee").unwrap();
    if amount < swap_fee {
        runtime::revert(Cep18Error::MintTooLow);
    }

    let balances_uref = get_balances_uref();
    let total_supply_uref = get_total_supply_uref();
    let mut new_balance = {
        let balance = read_balance_from(balances_uref, owner);
        balance
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    new_balance = new_balance
        .checked_sub(swap_fee)
        .ok_or(Cep18Error::Overflow)
        .unwrap_or_revert();

    let new_total_supply = {
        let total_supply: U256 = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    let dev_address: Key = utils::get_key("dev").unwrap();
    let new_dev_balance = {
        let balance = read_balance_from(balances_uref, dev_address);
        balance
            .checked_add(swap_fee)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    write_balance_to(balances_uref, dev_address, new_dev_balance);
    write_balance_to(balances_uref, owner, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
    events::record_event_dictionary(Event::Mint(Mint {
        recipient: owner,
        amount: amount - swap_fee,
    }));
    // emit cep47 dotoracle mint event
    let mut event = BTreeMap::new();
    let package_hash = runtime::get_key(PACKAGE_HASH).unwrap_or_revert();
    event.insert("contract_package_hash", package_hash.to_string());
    event.insert("event_type", "mint".to_string());
    event.insert("mint_id", mintid);
    let _: URef = storage::new_uref(event);

    if swap_fee > U256::zero() {
        events::record_event_dictionary(Event::Mint(Mint {
            recipient: dev_address,
            amount: swap_fee,
        }));
    }
}

#[no_mangle]
pub extern "C" fn change_dev() {
    sec_check(vec![SecurityBadge::Admin]);
    let dev: Key = runtime::get_named_arg("dev");
    utils::set_key("dev", dev);
}

#[no_mangle]
pub extern "C" fn change_swap_fee() {
    sec_check(vec![SecurityBadge::Admin]);
    let swap_fee: U256 = runtime::get_named_arg("swap_fee");
    utils::set_key("swap_fee", swap_fee);
}

#[no_mangle]
pub extern "C" fn request_bridge_back() {
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let fee: U256 = runtime::get_named_arg("fee");
    let to_chainid: U256 = runtime::get_named_arg("to_chainid");
    let receiver_address: String = runtime::get_named_arg("receiver_address");
    // let id: String = runtime::get_named_arg("id");

    let swap_fee: U256 = utils::get_key("swap_fee").unwrap();
    if swap_fee != fee {
        runtime::revert(Cep18Error::InvalidFee);
    }
    let dev_address: Key = utils::get_key("dev").unwrap();

    let request_index: U256 = utils::get_key("request_index").unwrap();
    let next_index = request_index + U256::one();

    let request_amount_after_fee = amount
        .checked_sub(fee)
        .ok_or(Cep18Error::RequestAmountTooLow)
        .unwrap_or_revert();

    utils::set_key("request_index", next_index);
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();

    if fee > U256::zero() {
        transfer_balance(owner, dev_address, fee).unwrap_or_revert();
        events::record_event_dictionary(Event::Transfer(Transfer {
            sender: owner,
            recipient: dev_address,
            amount: fee,
        }));
    }

    _burn_token(owner, request_amount_after_fee);
    // emit event request bridge and save request data
    utils::write_dictionary_value_from_key(
        "request_map",
        &request_index.to_string(),
        events::RequestBridgeBackData {
            to_chainid,
            from: owner,
            to: receiver_address,
            amount: request_amount_after_fee,
        },
    );
    let mut event = BTreeMap::new();
    let package_hash = runtime::get_key(PACKAGE_HASH).unwrap_or_revert();
    event.insert("contract_package_hash", package_hash.to_string());
    event.insert("event_type", "request_bridge_back".to_string());
    event.insert("request_index", request_index.to_string());
    let _: URef = storage::new_uref(event);
}

#[no_mangle]
pub extern "C" fn burn() {
    // disable direct burn of token
    // if 0 == read_from::<u8>(ENABLE_MINT_BURN) {
    //     revert(Cep18Error::MintBurnDisabled);
    // }
    // sec_check(vec![
    //     SecurityBadge::Admin,
    //     SecurityBadge::Burner,
    //     SecurityBadge::MintAndBurn,
    // ]);

    // let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    // let amount: U256 = runtime::get_named_arg(AMOUNT);
    // _burn_token(owner, amount);
}

#[no_mangle]
pub extern "C" fn exchange_from_old_token() {
    let old_token_package_hash: Key = utils::get_key("old_token_package_hash").unwrap();
    let amount: U256 = runtime::get_named_arg("amount");
    let caller = get_immediate_caller_address().unwrap_or_revert();
    let package_hash = runtime::get_key(PACKAGE_HASH).unwrap_or_revert();
    // transfer in and burn
    let _: () = runtime::call_versioned_contract(
        old_token_package_hash.into_hash().unwrap().into(),
        None,
        "transfer_from",
        runtime_args! {
            "owner" => caller,
            "amount" => amount,
            "recipient" => package_hash
        },
    );

    // burn
    let _: () = runtime::call_versioned_contract(
        old_token_package_hash.into_hash().unwrap().into(),
        None,
        "burn",
        runtime_args! {
            "amount" => amount
        },
    );

    // mint new token
    let balances_uref = get_balances_uref();
    let total_supply_uref = get_total_supply_uref();
    let new_balance = {
        let balance = read_balance_from(balances_uref, caller);
        balance
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };

    let new_total_supply = {
        let total_supply: U256 = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };

    write_balance_to(balances_uref, caller, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
    events::record_event_dictionary(Event::Mint(Mint {
        recipient: caller,
        amount,
    }));
}

fn _burn_token(owner: Key, amount: U256) {
    let balances_uref = get_balances_uref();
    let total_supply_uref = get_total_supply_uref();
    let new_balance = {
        let balance = read_balance_from(balances_uref, owner);
        balance
            .checked_sub(amount)
            .ok_or(Cep18Error::InsufficientBalance)
            .unwrap_or_revert()
    };
    let new_total_supply = {
        let total_supply = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_sub(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    write_balance_to(balances_uref, owner, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
    events::record_event_dictionary(Event::Burn(Burn { owner, amount }))
}

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    if get_key(ALLOWANCES).is_some() {
        revert(Cep18Error::AlreadyInitialized);
    }

    // adding key for dotoracle wrapped token
    let old_token_package_hash: Option<Key> = utils::get_optional_named_arg_with_user_errors(
        "old_token_package_hash",
        Cep18Error::InvalidOldToken,
    );
    if let Some(old_token_package_hash) = old_token_package_hash {
        utils::set_key("old_token_package_hash", old_token_package_hash);
    }

    // request index to track
    utils::set_key("request_index", U256::one());
    // request map to store request event data
    storage::new_dictionary("request_map").unwrap_or_revert();
    // mintids to store redeemed mintid to avoid double spend
    storage::new_dictionary("mintids").unwrap_or_revert();
    //dev wallet to receive fee
    let dev_address: Key = get_named_arg("dev");
    utils::set_key("dev", dev_address);
    let swap_fee: U256 = get_named_arg("swap_fee");
    utils::set_key("swap_fee", swap_fee);
    let origin_chainid: U256 = get_named_arg("origin_chainid");
    utils::set_key("origin_chainid", origin_chainid);
    let origin_contract_address: String = get_named_arg("origin_contract_address");
    utils::set_key("origin_contract_address", origin_contract_address);

    let package_hash = get_named_arg::<Key>(PACKAGE_HASH);
    put_key(PACKAGE_HASH, package_hash);
    storage::new_dictionary(ALLOWANCES).unwrap_or_revert();
    let balances_uref = storage::new_dictionary(BALANCES).unwrap_or_revert();
    let initial_supply = runtime::get_named_arg(TOTAL_SUPPLY);
    let caller = get_caller();
    write_balance_to(balances_uref, caller.into(), initial_supply);

    let security_badges_dict = storage::new_dictionary(SECURITY_BADGES).unwrap_or_revert();
    dictionary_put(
        security_badges_dict,
        &base64::encode(Key::from(get_caller()).to_bytes().unwrap_or_revert()),
        SecurityBadge::Admin,
    );

    let admin_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep18Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(MINTER_LIST, Cep18Error::InvalidMinterList);
    let burner_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(BURNER_LIST, Cep18Error::InvalidBurnerList);
    let mint_and_burn_list: Option<Vec<Key>> = utils::get_optional_named_arg_with_user_errors(
        MINT_AND_BURN_LIST,
        Cep18Error::InvalidMintAndBurnList,
    );

    init_events();

    if let Some(minter_list) = minter_list {
        for minter in minter_list {
            dictionary_put(
                security_badges_dict,
                &base64::encode(minter.to_bytes().unwrap_or_revert()),
                SecurityBadge::Minter,
            );
        }
    }
    if let Some(burner_list) = burner_list {
        for burner in burner_list {
            dictionary_put(
                security_badges_dict,
                &base64::encode(burner.to_bytes().unwrap_or_revert()),
                SecurityBadge::Burner,
            );
        }
    }
    if let Some(mint_and_burn_list) = mint_and_burn_list {
        for mint_and_burn in mint_and_burn_list {
            dictionary_put(
                security_badges_dict,
                &base64::encode(mint_and_burn.to_bytes().unwrap_or_revert()),
                SecurityBadge::MintAndBurn,
            );
        }
    }
    if let Some(admin_list) = admin_list {
        for admin in admin_list {
            dictionary_put(
                security_badges_dict,
                &base64::encode(admin.to_bytes().unwrap_or_revert()),
                SecurityBadge::Admin,
            );
        }
    }
}

/// Admin EntryPoint to manipulate the security access granted to users.
/// One user can only possess one access group badge.
/// Change strength: None > Admin > MintAndBurn > Burner > Minter
/// Change strength meaning by example: If user is added to both Minter and Admin they will be an
/// Admin, also if a user is added to Admin and None then they will be removed from having rights.
/// Beware: do not remove the last Admin because that will lock out all admin functionality.
#[no_mangle]
pub extern "C" fn change_security() {
    if 0 == read_from::<u8>(ENABLE_MINT_BURN) {
        revert(Cep18Error::MintBurnDisabled);
    }
    sec_check(vec![SecurityBadge::Admin]);
    let admin_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep18Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(MINTER_LIST, Cep18Error::InvalidMinterList);
    let burner_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(BURNER_LIST, Cep18Error::InvalidBurnerList);
    let mint_and_burn_list: Option<Vec<Key>> = utils::get_optional_named_arg_with_user_errors(
        MINT_AND_BURN_LIST,
        Cep18Error::InvalidMintAndBurnList,
    );
    let none_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(NONE_LIST, Cep18Error::InvalidNoneList);

    let mut badge_map: BTreeMap<Key, SecurityBadge> = BTreeMap::new();
    if let Some(minter_list) = minter_list {
        for account_key in minter_list {
            badge_map.insert(account_key, SecurityBadge::Minter);
        }
    }
    if let Some(burner_list) = burner_list {
        for account_key in burner_list {
            badge_map.insert(account_key, SecurityBadge::Burner);
        }
    }
    if let Some(mint_and_burn_list) = mint_and_burn_list {
        for account_key in mint_and_burn_list {
            badge_map.insert(account_key, SecurityBadge::MintAndBurn);
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

    utils::change_sec_badge(&badge_map);
    events::record_event_dictionary(Event::ChangeSecurity(ChangeSecurity {
        admin: get_immediate_caller_address().unwrap_or_revert(),
        sec_change_map: badge_map,
    }));
}

#[no_mangle]
pub extern "C" fn migrate() {}

pub fn install_contract() {
    let name: String = runtime::get_named_arg(NAME);
    let hash_key_name = format!("{HASH_KEY_NAME_PREFIX}{name}");
    // If this is the first deployment
    if !runtime::has_key(&hash_key_name) {
        let symbol: String = runtime::get_named_arg(SYMBOL);
        let decimals: u8 = runtime::get_named_arg(DECIMALS);
        let total_supply: U256 = runtime::get_named_arg(TOTAL_SUPPLY);
        let events_mode: u8 = utils::get_optional_named_arg_with_user_errors(
            EVENTS_MODE,
            Cep18Error::InvalidEventsMode,
        )
        .unwrap_or(1u8);

        let dev_address: Key = get_named_arg("dev");
        let swap_fee: U256 = get_named_arg("swap_fee");
        let origin_chainid: U256 = get_named_arg("origin_chainid");
        let origin_contract_address: String = get_named_arg("origin_contract_address");
        let old_token_package_hash: Option<Key> = utils::get_optional_named_arg_with_user_errors(
            "old_token_package_hash",
            Cep18Error::InvalidOldToken,
        );

        let admin_list: Option<Vec<Key>> = utils::get_optional_named_arg_with_user_errors(
            ADMIN_LIST,
            Cep18Error::InvalidAdminList,
        );
        let minter_list: Option<Vec<Key>> = utils::get_optional_named_arg_with_user_errors(
            MINTER_LIST,
            Cep18Error::InvalidMinterList,
        );
        let burner_list: Option<Vec<Key>> = utils::get_optional_named_arg_with_user_errors(
            BURNER_LIST,
            Cep18Error::InvalidBurnerList,
        );
        let mint_and_burn_list: Option<Vec<Key>> = utils::get_optional_named_arg_with_user_errors(
            MINT_AND_BURN_LIST,
            Cep18Error::InvalidMintAndBurnList,
        );

        // by default it is mint and burnable
        let enable_mint_burn: u8 = utils::get_optional_named_arg_with_user_errors(
            ENABLE_MINT_BURN,
            Cep18Error::InvalidEnableMBFlag,
        )
        .unwrap_or(0);

        let mut named_keys = NamedKeys::new();
        named_keys.insert(NAME.to_string(), storage::new_uref(name.clone()).into());
        named_keys.insert(SYMBOL.to_string(), storage::new_uref(symbol).into());
        named_keys.insert(DECIMALS.to_string(), storage::new_uref(decimals).into());
        named_keys.insert(
            TOTAL_SUPPLY.to_string(),
            storage::new_uref(total_supply).into(),
        );
        named_keys.insert(
            EVENTS_MODE.to_string(),
            storage::new_uref(events_mode).into(),
        );
        named_keys.insert(
            ENABLE_MINT_BURN.to_string(),
            storage::new_uref(enable_mint_burn).into(),
        );
        let mut entry_points = generate_entry_points();
        if old_token_package_hash.is_some() {
            entry_points.add_entry_point(exchange_token());
        }

        let (contract_hash, contract_version) = storage::new_contract(
            entry_points,
            Some(named_keys),
            Some(hash_key_name.clone()),
            Some(format!("{ACCESS_KEY_NAME_PREFIX}{name}")),
        );
        let package_hash = runtime::get_key(&hash_key_name).unwrap_or_revert();

        // Store contract_hash and contract_version under the keys CONTRACT_NAME and
        // CONTRACT_VERSION
        runtime::put_key(
            &format!("{CONTRACT_NAME_PREFIX}{name}"),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{CONTRACT_VERSION_PREFIX}{name}"),
            storage::new_uref(contract_version).into(),
        );
        // Call contract to initialize it
        let mut init_args = runtime_args! {
            TOTAL_SUPPLY => total_supply,
            PACKAGE_HASH => package_hash,
            "dev" => dev_address,
            "swap_fee" => swap_fee,
            "origin_chainid" => origin_chainid,
            "origin_contract_address" => origin_contract_address
        };

        if let Some(old_token_package_hash) = old_token_package_hash {
            init_args
                .insert("old_token_package_hash", old_token_package_hash)
                .unwrap_or_revert();
        }

        if let Some(admin_list) = admin_list {
            init_args.insert(ADMIN_LIST, admin_list).unwrap_or_revert();
        }
        if let Some(minter_list) = minter_list {
            init_args
                .insert(MINTER_LIST, minter_list)
                .unwrap_or_revert();
        }
        if let Some(burner_list) = burner_list {
            init_args
                .insert(BURNER_LIST, burner_list)
                .unwrap_or_revert();
        }
        if let Some(mint_and_burn_list) = mint_and_burn_list {
            init_args
                .insert(MINT_AND_BURN_LIST, mint_and_burn_list)
                .unwrap_or_revert();
        }

        runtime::call_contract::<()>(contract_hash, INIT_ENTRY_POINT_NAME, init_args);
    } else {
        let package_hash: ContractPackageHash = runtime::get_key(&hash_key_name)
            .unwrap_or_revert()
            .into_hash()
            .unwrap()
            .into();

        let (contract_hash, _) = storage::add_contract_version(
            package_hash,
            generate_entry_points(),
            Default::default(),
        );

        // update contract hash
        runtime::put_key(
            &format!("{CONTRACT_NAME_PREFIX}{name}"),
            contract_hash.into(),
        );
    }
}

#[no_mangle]
pub extern "C" fn call() {
    install_contract()
}
