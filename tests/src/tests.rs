use crate::erc20::{
    account::{ALI as ALI2, BOB, JOE},
    token_cfg, Sender, Token,
};
use casper_engine_test_support::AccountHash;



#[test]
fn test_erc20_deploy() {

let ALI: AccountHash = ALI2.to_account_hash();
    let token = Token::deployed();
    assert_eq!(token.name(), token_cfg::NAME);
    assert_eq!(token.symbol(), token_cfg::SYMBOL);
    assert_eq!(token.decimals(), token_cfg::DECIMALS);
    assert_eq!(token.balance_of(ALI), token_cfg::total_supply());
    assert_eq!(token.balance_of(BOB), 0.into());
    assert_eq!(token.allowance(ALI, ALI), 0.into());
    assert_eq!(token.allowance(ALI, BOB), 0.into());
    assert_eq!(token.allowance(BOB, ALI), 0.into());
    assert_eq!(token.allowance(BOB, BOB), 0.into());
}

#[test]
fn test_erc20_transfer() {

let ALI: AccountHash = ALI2.to_account_hash();
    let amount = 10.into();
    let mut token = Token::deployed();
    token.transfer(BOB, amount, Sender(ALI));
    assert_eq!(token.balance_of(ALI), token_cfg::total_supply() - amount);
    assert_eq!(token.balance_of(BOB), amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_too_much() {
    let amount = 1.into();
    let mut token = Token::deployed();
    token.transfer(ALI2.to_account_hash(), amount, Sender(BOB));
}

#[test]
fn test_erc20_approve() {
    let amount = 10.into();

let ALI: AccountHash = ALI2.to_account_hash();
    let mut token = Token::deployed();
    token.approve(BOB, amount, Sender(ALI));
    assert_eq!(token.balance_of(ALI), token_cfg::total_supply());
    assert_eq!(token.balance_of(BOB), 0.into());
    assert_eq!(token.allowance(ALI, BOB), amount);
    assert_eq!(token.allowance(BOB, ALI), 0.into());
}

#[test]
fn test_erc20_transfer_from() {
    let allowance = 10.into();

let ALI: AccountHash = ALI2.to_account_hash();
    let amount = 3.into();
    let mut token = Token::deployed();
    token.approve(BOB, allowance, Sender(ALI));
    token.transfer_from(ALI, JOE, amount, Sender(BOB));
    assert_eq!(token.balance_of(ALI), token_cfg::total_supply() - amount);
    assert_eq!(token.balance_of(BOB), 0.into());
    assert_eq!(token.balance_of(JOE), amount);
    assert_eq!(token.allowance(ALI, BOB), allowance - amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_from_too_much() {

let ALI: AccountHash = ALI2.to_account_hash();
    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
    let mut token = Token::deployed();
    token.transfer_from(ALI, JOE, amount, Sender(BOB));
}
