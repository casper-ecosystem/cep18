use crate::{
    api::{self, Api},
    error::Error,
    env, erc20
};
use casperlabs_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{ContractRef, Key, U512};


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
pub extern "C" fn erc20() {
    if env::is_initialized(env::ERC20_CONTRACT_NAME) {
        erc20::handle().unwrap_or_revert();
    } else {
        erc20::constructor();
        env::mark_as_initialized(env::ERC20_CONTRACT_NAME);
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
