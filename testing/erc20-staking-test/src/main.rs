#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
};
use core::ops::{Deref, DerefMut};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc20::{
    constants::{ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME},
    Address, Error, ERC20,
};
use casper_types::{
    account::AccountHash, CLType, CLTyped, CLValue, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256,
};

const MINT_ENTRY_POINT_NAME: &str = "mint";
const BURN_ENTRY_POINT_NAME: &str = "burn";

const CREATE_STAKE_ENTRY_POINT_NAME: &str = "create_stake";
const REMOVE_STAKE_ENTRY_POINT_NAME: &str = "remove_stake";
const STAKE_OF_ENTRY_POINT_NAME: &str = "stake_of";
const TOTAL_STAKES_ENTRY_POINT_NAME: &str = "total_stakes";
const IS_STAKER_ENTRY_POINT_NAME: &str = "is_staker";
const ADD_STAKER_ENTRY_POINT_NAME: &str = "add_staker";
const REWARDS_OF_ENTRY_POINT_NAME: &str = "rewards_of";
const TOTAL_REWARDS_ENTRY_POINT_NAME: &str = "total_rewards";
const CALCULATE_REWARDS_ENTRY_POINT_NAME: &str = "calculate_rewards";
const DISTRIBUTE_REWARDS_ENTRY_POINT_NAME: &str = "distribute_rewards";
const WITHDRAW_REWARD_ENTRY_POINT_NAME: &str = "withdraw_reward";

/// "erc20" is not mentioned here intentionally as the functionality is not compatible with ERC20
/// token standard.
const TEST_CONTRACT_KEY_NAME: &str = "test_contract";
const TOKEN_NAME: &str = "CasperStakingTest";
const TOKEN_SYMBOL: &str = "CSPRSTAKING";
const TOKEN_DECIMALS: u8 = 8;
const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;

const TOKEN_OWNER_ADDRESS_1: Address = Address::Account(AccountHash::new([42; 32]));
const TOKEN_OWNER_AMOUNT_1: u64 = 1_000_000;
const TOKEN_OWNER_ADDRESS_2: Address = Address::Contract(ContractPackageHash::new([42; 32]));
const TOKEN_OWNER_AMOUNT_2: u64 = 2_000_000;

#[derive(Default)]
struct TestStakingToken {
    erc20: ERC20,
}

impl TestStakingToken {
    pub fn install() -> Result<TestStakingToken, Error> {
        let name: String = TOKEN_NAME.to_string();
        let symbol: String = TOKEN_SYMBOL.to_string();
        let decimals = TOKEN_DECIMALS;
        let total_supply = U256::from(TOKEN_TOTAL_SUPPLY);

        let mut entry_points = EntryPoints::new();

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
        // rewards_of_entrypoint
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
            // NOTE: For security reasons never use this entrypoint definition in a production
            // contract. This is marks the entry point as public.
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
            // NOTE: For security reasons never use this entrypoint definition in a production
            // contract. This is marks the entry point as public.
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
            // NOTE: For security reasons never use this entrypoint definition in a production
            // contract. This is marks the entry point as public.
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
            // NOTE: For security reasons never use this entrypoint definition in a production
            // contract. This is marks the entry point as public.
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        let rewards_of_entrypoint = EntryPoint::new(
            REWARDS_OF_ENTRY_POINT_NAME,
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
        let total_rewards_entrypoint = EntryPoint::new(
            TOTAL_REWARDS_ENTRY_POINT_NAME,
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
        let calculate_rewards_entrypoint = EntryPoint::new(
            CALCULATE_REWARDS_ENTRY_POINT_NAME,
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
        let distribute_rewards_entrypoint = EntryPoint::new(
            DISTRIBUTE_REWARDS_ENTRY_POINT_NAME,
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
        let withdraw_reward_entrypoint = EntryPoint::new(
            WITHDRAW_REWARD_ENTRY_POINT_NAME,
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

        entry_points.add_entry_point(casper_erc20::entry_points::total_supply());
        entry_points.add_entry_point(casper_erc20::entry_points::balance_of());
        entry_points.add_entry_point(mint_entrypoint);
        entry_points.add_entry_point(burn_entrypoint);
        entry_points.add_entry_point(create_stake_entrypoint);
        entry_points.add_entry_point(remove_stake_entrypoint);
        entry_points.add_entry_point(stake_of_entrypoint);
        entry_points.add_entry_point(total_stakes_entrypoint);
        entry_points.add_entry_point(is_staker_entrypoint);
        entry_points.add_entry_point(add_staker_entrypoint);
        entry_points.add_entry_point(rewards_of_entrypoint);
        entry_points.add_entry_point(total_rewards_entrypoint);
        entry_points.add_entry_point(calculate_rewards_entrypoint);
        entry_points.add_entry_point(distribute_rewards_entrypoint);
        entry_points.add_entry_point(withdraw_reward_entrypoint);

        // Caution: This test uses `install_custom` without providing default entrypoints as
        // described by ERC20 token standard.
        //
        // This is unsafe and this test contract is not a ERC20 token standard-compliant token.
        // Contract developers should use example/erc20 contract instead as a template for writing
        // their own tokens.
        let erc20 = ERC20::install_custom(
            name,
            symbol,
            decimals,
            total_supply,
            TEST_CONTRACT_KEY_NAME,
            entry_points,
        )?;
        Ok(TestStakingToken { erc20 })
    }
}

impl Deref for TestStakingToken {
    type Target = ERC20;

    fn deref(&self) -> &Self::Target {
        &self.erc20
    }
}

impl DerefMut for TestStakingToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.erc20
    }
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val = TestStakingToken::default().total_supply();
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let val = TestStakingToken::default().balance_of(address);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn mint() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .mint(owner, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn burn() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .burn(owner, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn create_stake() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .create_stake(owner, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn remove_stake() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .remove_stake(owner, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn stake_of() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .stake_of(owner)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn total_stakes() {
    TestStakingToken::default()
        .total_stakes()
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn is_staker() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .is_staker(owner)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn add_staker() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .add_staker(owner)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn reward_of() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .reward_of(owner)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn total_rewards() {
    TestStakingToken::default()
        .total_rewards()
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn calculate_reward() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .calculate_rewards(owner)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn distribute_rewards() {
    TestStakingToken::default()
        .distribute_rewards()
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn withdraw_reward() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    TestStakingToken::default()
        .withdraw_reward(owner)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let mut test_token = TestStakingToken::install().unwrap_or_revert();

    test_token
        .mint(TOKEN_OWNER_ADDRESS_1, U256::from(TOKEN_OWNER_AMOUNT_1))
        .unwrap_or_revert();

    test_token
        .mint(TOKEN_OWNER_ADDRESS_2, U256::from(TOKEN_OWNER_AMOUNT_2))
        .unwrap_or_revert();
}
