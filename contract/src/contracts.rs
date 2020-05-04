use crate::{
    env, erc20,
    error::Error,
    input_parser::{self, Input},
};
use casperlabs_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casperlabs_types::CLValue;
use logic::ERC20Trait;

#[no_mangle]
pub extern "C" fn call() {
    match input_parser::from_args() {
        Input::Deploy(initial_balance) => {
            env::deploy_token(initial_balance);
            env::deploy_indirect();
        }
        _ => runtime::revert(Error::UnknownDeployCommand),
    }
}

#[no_mangle]
pub extern "C" fn erc20_indirect() {
    let token = input_parser::destination_contract();
    match input_parser::from_args() {
        Input::Transfer(recipient, amount) => {
            let args = (input_parser::TRANSFER, recipient, amount);
            runtime::call_contract::<_, ()>(token, args);
        }
        Input::TransferFrom(owner, recipient, amount) => {
            let args = (input_parser::TRANSFER_FROM, owner, recipient, amount);
            runtime::call_contract::<_, ()>(token, args);
        }
        Input::Approve(spender, amount) => {
            let args = (input_parser::APPROVE, spender, amount);
            runtime::call_contract::<_, ()>(token, args);
        }
        _ => runtime::revert(Error::UnknownIndirectCommand),
    }
}

#[no_mangle]
pub extern "C" fn erc20() {
    if env::is_initialized(env::ERC20_CONTRACT_NAME) {
        handle_erc20().unwrap_or_revert();
    } else {
        init_erc20();
        env::mark_as_initialized(env::ERC20_CONTRACT_NAME);
    }
}

pub fn handle_erc20() -> Result<(), Error> {
    let mut token = erc20::ERC20Token;
    match input_parser::from_args() {
        Input::Transfer(recipient, amount) => token
            .transfer(&runtime::get_caller(), &recipient, amount)
            .map_err(Error::from),
        Input::Approve(spender, amount) => {
            token.approve(&runtime::get_caller(), &spender, amount);
            Ok(())
        }
        Input::TransferFrom(owner, recipient, amount) => token
            .transfer_from(&runtime::get_caller(), &owner, &recipient, amount)
            .map_err(Error::from),
        Input::BalanceOf(address) => {
            runtime::ret(CLValue::from_t(token.balance_of(&address)).unwrap_or_revert())
        }
        Input::Allowance(owner, spender) => {
            runtime::ret(CLValue::from_t(token.allowance(&owner, &spender)).unwrap_or_revert())
        }
        Input::TotalSupply => {
            runtime::ret(CLValue::from_t(token.total_supply()).unwrap_or_revert())
        }
        _ => Err(Error::UnknownErc20CallCommand),
    }
}

pub fn init_erc20() {
    if let Input::InitErc20(amount) = input_parser::from_args() {
        let mut token = erc20::ERC20Token;
        token.mint(&runtime::get_caller(), amount);
    } else {
        runtime::revert(Error::UnknownErc20ConstructorCommand);
    }
}
