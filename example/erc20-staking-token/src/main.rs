#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{string::String, vec};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc20::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
        ERC20_TOKEN_CONTRACT_KEY_NAME, NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME,
        RECIPIENT_RUNTIME_ARG_NAME, SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME,
        TOTAL_SUPPLY_RUNTIME_ARG_NAME,
    },
    Address, ERC20,
};
use casper_types::{
    CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType, Parameter, U256,
};

const MINT_ENTRY_POINT_NAME: &str = "mint";
const BURN_ENTRY_POINT_NAME: &str = "burn";

const CREATE_STAKE_ENTRY_POINT_NAME: &str = "create_stake";
const REMOVE_STAKE_ENTRY_POINT_NAME: &str = "remove_stake";
const STAKE_OF_ENTRY_POINT_NAME: &str = "stake_of";
const TOTAL_STAKES_ENTRY_POINT_NAME: &str = "total_stakes";
const IS_STAKER_ENTRY_POINT_NAME: &str = "is_staker";
const ADD_STAKER_ENTRY_POINT_NAME: &str = "add_staker";
const REWARD_OF_ENTRY_POINT_NAME: &str = "reward_of";
const TOTAL_REWARDS_ENTRY_POINT_NAME: &str = "total_rewards";
const CALCULATE_REWARDS_ENTRY_POINT_NAME: &str = "calculate_rewards";
const DISTRIBUTE_REWARDS_ENTRY_POINT_NAME: &str = "distribute_rewards";
const WITHDRAW_REWARD_ENTRY_POINT_NAME: &str = "withdraw_reward";

/* Default ERC20 entry points */

#[no_mangle]
pub extern "C" fn name() {
    let name = ERC20::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let symbol = ERC20::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    let decimals = ERC20::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let total_supply = ERC20::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC20::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default()
        .transfer(recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let val = ERC20::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
}

/* Mint / burn entry points */

#[no_mangle]
pub extern "C" fn mint() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default().mint(owner, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn burn() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default().burn(owner, amount).unwrap_or_revert();
}

/* Staking entry points */

#[no_mangle]
pub extern "C" fn create_stake() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .create_stake(owner, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn remove_stake() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .remove_stake(owner, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn stake_of() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    ERC20::default().stake_of(owner).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn total_stakes() {
    ERC20::default().total_stakes().unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn is_staker() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    ERC20::default().is_staker(owner).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn add_staker() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    ERC20::default().add_staker(owner).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn reward_of() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    ERC20::default().reward_of(owner).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn total_rewards() {
    ERC20::default().total_rewards().unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn calculate_rewards() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    ERC20::default().calculate_rewards(owner).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn distribute_rewards() {
    ERC20::default().distribute_rewards().unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn withdraw_reward() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    ERC20::default().withdraw_reward(owner).unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
    let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);

    let mint_entrypoint = EntryPoint::new(
        MINT_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        // NOTE: For security reasons never use this entrypoint definition in a production
        // contract. This is marks the entry point as public.
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let burn_entrypoint = EntryPoint::new(
        BURN_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        // NOTE: For security reasons never use this entrypoint definition in a production
        // contract. This is marks the entry point as public.
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // Entry points related to staking
    // create_stake_entrypoint
    // remove_stake_entrypoint
    // stake_of_entrypoint
    // total_stakes_entrypoint
    // is_staker_entrypoint
    // add_staker_entrypoint
    // reward_of_entrypoint
    // total_rewards_entrypoint
    // calculate_rewards_entrypoint
    // distribute_rewards_entrypoint
    // withdraw_reward_entrypoint

    let create_stake_entrypoint = EntryPoint::new(
        CREATE_STAKE_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        // NOTE: For security reasons never use this entrypoint definition in a production
        // contract. This is marks the entry point as public.
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let remove_stake_entrypoint = EntryPoint::new(
        REMOVE_STAKE_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        // NOTE: For security reasons never use this entrypoint definition in a production
        // contract. This is marks the entry point as public.
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let stake_of_entrypoint = EntryPoint::new(
        STAKE_OF_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let total_stakes_entrypoint = EntryPoint::new(
        TOTAL_STAKES_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let is_staker_entrypoint = EntryPoint::new(
        IS_STAKER_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let add_staker_entrypoint = EntryPoint::new(
        ADD_STAKER_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let reward_of_entrypoint = EntryPoint::new(
        REWARD_OF_ENTRY_POINT_NAME,
        vec![Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let total_rewards_entrypoint = EntryPoint::new(
        TOTAL_REWARDS_ENTRY_POINT_NAME,
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let calculate_rewards_entrypoint = EntryPoint::new(
        CALCULATE_REWARDS_ENTRY_POINT_NAME,
        vec![Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let distribute_rewards_entrypoint = EntryPoint::new(
        DISTRIBUTE_REWARDS_ENTRY_POINT_NAME,
        vec![Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type())],
        CLType::Unit,
        // NOTE: For security reasons never use this entrypoint definition in a production
        // contract. This is marks the entry point as public.
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let withdraw_reward_entrypoint = EntryPoint::new(
        WITHDRAW_REWARD_ENTRY_POINT_NAME,
        vec![Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type())],
        CLType::Unit,
        // NOTE: For security reasons never use this entrypoint definition in a production
        // contract. This is marks the entry point as public.
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points = casper_erc20::entry_points::default();

    entry_points.add_entry_point(mint_entrypoint);
    entry_points.add_entry_point(burn_entrypoint);

    entry_points.add_entry_point(create_stake_entrypoint);
    entry_points.add_entry_point(remove_stake_entrypoint);
    entry_points.add_entry_point(stake_of_entrypoint);
    entry_points.add_entry_point(total_stakes_entrypoint);
    entry_points.add_entry_point(is_staker_entrypoint);
    entry_points.add_entry_point(add_staker_entrypoint);
    entry_points.add_entry_point(reward_of_entrypoint);
    entry_points.add_entry_point(total_rewards_entrypoint);
    entry_points.add_entry_point(calculate_rewards_entrypoint);
    entry_points.add_entry_point(distribute_rewards_entrypoint);
    entry_points.add_entry_point(withdraw_reward_entrypoint);

    let _token = ERC20::install_custom(
        name,
        symbol,
        decimals,
        total_supply,
        ERC20_TOKEN_CONTRACT_KEY_NAME,
        entry_points,
    )
    .unwrap_or_revert();
}
