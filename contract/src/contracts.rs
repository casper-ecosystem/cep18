use crate::{
    api::{self, Api},
    env, erc20,
    error::Error,
};
use casperlabs_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casperlabs_types::CLValue;
use logic::ERC20Trait;

#[no_mangle]
pub extern "C" fn call() {
    match Api::from_args() {
        Api::Deploy(initial_balance) => {
            env::deploy_token(initial_balance);
            env::deploy_proxy();
        }
        _ => runtime::revert(Error::UnknownDeployCommand),
    }
}

#[no_mangle]
pub extern "C" fn erc20_proxy() {
    let token = api::destination_contract();
    match Api::from_args() {
        Api::Transfer(recipient, amount) => {
            let args = (api::TRANSFER, recipient, amount);
            runtime::call_contract::<_, ()>(token, args);
        }
        Api::TransferFrom(owner, recipient, amount) => {
            let args = (api::TRANSFER_FROM, owner, recipient, amount);
            runtime::call_contract::<_, ()>(token, args);
        }
        Api::Approve(spender, amount) => {
            let args = (api::APPROVE, spender, amount);
            runtime::call_contract::<_, ()>(token, args);
        }
        _ => runtime::revert(Error::UnknownProxyCommand),
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
    match Api::from_args() {
        Api::Transfer(recipient, amount) => token
            .transfer(&runtime::get_caller(), &recipient, amount)
            .map_err(Error::from),
        Api::Approve(spender, amount) => {
            token.approve(&runtime::get_caller(), &spender, amount);
            Ok(())
        }
        Api::TransferFrom(owner, recipient, amount) => token
            .transfer_from(&runtime::get_caller(), &owner, &recipient, amount)
            .map_err(Error::from),
        Api::BalanceOf(address) => {
            runtime::ret(CLValue::from_t(token.balance_of(&address)).unwrap_or_revert())
        }
        Api::Allowance(owner, spender) => {
            runtime::ret(CLValue::from_t(token.allowance(&owner, &spender)).unwrap_or_revert())
        }
        Api::TotalSupply => runtime::ret(CLValue::from_t(token.total_supply()).unwrap_or_revert()),
        _ => Err(Error::UnknownErc20CallCommand),
    }
}

pub fn init_erc20() {
    if let Api::InitErc20(amount) = Api::from_args() {
        let mut token = erc20::ERC20Token;
        token.mint(&runtime::get_caller(), amount);
    } else {
        runtime::revert(Error::UnknownErc20ConstructorCommand);
    }
}
