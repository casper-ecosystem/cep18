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

/// "erc20" is not mentioned here intentionally as the functionality is not compatible with ERC20
/// token standard.
const TEST_CONTRACT_KEY_NAME: &str = "test_contract";
const TOKEN_NAME: &str = "CasperTest";
const TOKEN_SYMBOL: &str = "CSPRT";
const TOKEN_DECIMALS: u8 = 8;
const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;

const TOKEN_OWNER_ADDRESS_1: Address = Address::Account(AccountHash::new([42; 32]));
const TOKEN_OWNER_AMOUNT_1: u64 = 1_000_000;
const TOKEN_OWNER_ADDRESS_2: Address = Address::Contract(ContractPackageHash::new([42; 32]));
const TOKEN_OWNER_AMOUNT_2: u64 = 2_000_000;

#[derive(Default)]
struct TestToken {
    erc20: ERC20,
}

impl TestToken {
    pub fn install() -> Result<TestToken, Error> {
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
            // NOTE: `Public` access rights allows any user and context to call this entrypoint.
            // If security is required we suggest developers to implement additional security code
            // inside public entrypoints.
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
            // NOTE: `Public` access rights allows any user and context to call this entrypoint.
            // If security is required we suggest developers to implement additional security code
            // inside public entrypoints.
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );

        entry_points.add_entry_point(casper_erc20::entry_points::total_supply());
        entry_points.add_entry_point(casper_erc20::entry_points::balance_of());
        entry_points.add_entry_point(mint_entrypoint);
        entry_points.add_entry_point(burn_entrypoint);

        // Caution: This test uses `install_custom` without providing default entrypoints as
        // described by ERC20 token standard.
        //
        // This test contract is not a ERC20 token standard-compliant token.
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
        Ok(TestToken { erc20 })
    }
}

impl Deref for TestToken {
    type Target = ERC20;

    fn deref(&self) -> &Self::Target {
        &self.erc20
    }
}

impl DerefMut for TestToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.erc20
    }
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val = TestToken::default().total_supply();
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let val = TestToken::default().balance_of(address);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn mint() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    TestToken::default().mint(owner, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn burn() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    TestToken::default().burn(owner, amount).unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let mut test_token = TestToken::install().unwrap_or_revert();

    test_token
        .mint(TOKEN_OWNER_ADDRESS_1, U256::from(TOKEN_OWNER_AMOUNT_1))
        .unwrap_or_revert();

    test_token
        .mint(TOKEN_OWNER_ADDRESS_2, U256::from(TOKEN_OWNER_AMOUNT_2))
        .unwrap_or_revert();
}
