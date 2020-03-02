use alloc::string::String;

use casperlabs_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casperlabs_types::{account::PublicKey, bytesrepr::{FromBytes, Error as ApiError}, CLTyped, ContractRef, URef, U512};

use crate::error::Error;

pub const DEPLOY: &str = "deploy";
pub const INIT_ERC20: &str = "init_erc20";
pub const BALANCE_OF: &str = "balance_of";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const TRANSFER: &str = "transfer";
pub const TRANSFER_FROM: &str = "transfer_from";
pub const APPROVE: &str = "approve";
pub const ALLOWANCE: &str = "allowance";

pub enum Api {
    Deploy(U512),
    InitErc20(U512),
    Transfer(PublicKey, U512),
    TransferFrom(PublicKey, PublicKey, U512),
    Approve(PublicKey, U512),
    BalanceOf(PublicKey),
    Allowance(PublicKey, PublicKey),
    TotalSupply,
}

impl Api {
    pub fn from_args() -> Api {
        let method: String = method_name();
        match method.as_str() {
            DEPLOY          => Api::Deploy(get_arg(1)),
            INIT_ERC20      => Api::InitErc20(get_arg(1)),
            TRANSFER        => Api::Transfer(get_arg(1), get_arg(2)),
            TRANSFER_FROM   => Api::TransferFrom(get_arg(1), get_arg(2), get_arg(3)),
            APPROVE         => Api::Approve(get_arg(1), get_arg(2)),
            BALANCE_OF      => Api::BalanceOf(get_arg(1)),
            ALLOWANCE       => Api::Allowance(get_arg(1), get_arg(2)),
            TOTAL_SUPPLY    => Api::TotalSupply,
            _ => runtime::revert(Error::UnknownApiCommand),
        }
    }
}

pub fn destination_contract() -> ContractRef {
    let (_, hash): (String, [u8; 32]) = runtime::get_arg(0)
            .unwrap_or_revert_with(Error::missing_argument(0))
            .unwrap_or_revert_with(Error::invalid_argument(0));
    ContractRef::Hash(hash)
}

fn get_arg<T: CLTyped + FromBytes>(i: u32) -> T {
    runtime::get_arg(i)
        .unwrap_or_revert_with(Error::missing_argument(i))
        .unwrap_or_revert_with(Error::invalid_argument(i))
}

fn method_name() -> String {
    let maybe_argument: Result<String, ApiError> = 
        runtime::get_arg(0).unwrap_or_revert_with(Error::missing_argument(0));
    match maybe_argument {
        Ok(method) => method,
        Err(_) => {
            let (method, _): (String, [u8; 32]) = runtime::get_arg(0)
                .unwrap_or_revert_with(Error::missing_argument(0))
                .unwrap_or_revert_with(Error::invalid_argument(0));
            method
        }
    }
}

