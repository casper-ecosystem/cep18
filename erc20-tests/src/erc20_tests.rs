use casper_engine_test_support::AccountHash;
use casper_types::U256;
use test_env::{Sender, TestEnv};

use crate::erc20_instance::ERC20Instance;

const NAME: &str = "my_token";
const SYMBOL: &str = "MYT";
const DECIMALS: u8 = 18;
const INIT_TOTAL_SUPPLY: u64 = 1000;

fn deploy() -> (TestEnv, ERC20Instance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = ERC20Instance::new(
        &env,
        NAME,
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
    (env, token, owner)
}

#[test]
fn test_erc20_deploy() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.decimals(), DECIMALS);
    assert_eq!(token.total_supply(), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_erc20_transfer() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.transfer(Sender(owner), user, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(INIT_TOTAL_SUPPLY) - amount
    );
    assert_eq!(token.balance_of(user), amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_too_much() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = U256::from(INIT_TOTAL_SUPPLY) + U256::one();
    token.transfer(Sender(owner), user, amount);
}

#[test]
fn test_erc20_approve() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(Sender(owner), user, amount);
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_erc20_transfer_from() {
    let (env, token, owner) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 3.into();
    token.approve(Sender(owner), spender, allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(INIT_TOTAL_SUPPLY) - amount
    );
    assert_eq!(token.balance_of(spender), 0.into());
    assert_eq!(token.balance_of(recipient), amount);
    assert_eq!(token.allowance(owner, spender), allowance - amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_from_too_much() {
    let (env, token, owner) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 12.into();
    token.approve(Sender(owner), spender, allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_, token, owner) = deploy();
    token.constructor(
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
}
