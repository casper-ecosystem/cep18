use casperlabs_types::ApiError;
use erc20_logic::{ERC20BurnError, ERC20Trait, ERC20TransferError, ERC20TransferFromError};

#[repr(u16)]
pub enum Error {
    UnknownApiCommand = 1,                      // 65537
    UnknownDeployCommand = 2,                   // 65538
    UnknownProxyCommand = 3,                    // 65539
    UnknownErc20ConstructorCommand = 4,         // 65540
    UnknownErc20CallCommand = 5,                // 65541
    BalanceAssertionFailure = 6,                // 65542
    TotalSupplyAssertionFailure = 7,            // 65543
    AllowanceAssertionFailure = 8,              // 65544
    TransferFailureNotEnoughBalance = 9,        // 65545
    TransferFromFailureNotEnoughBalance = 10,   // 65546
    TransferFromFailureNotEnoughAllowance = 11, // 65547
    PurseTransferError = 12,                    // 65548
    LocalPurseKeyMissing = 13,                  // 65549
    NotAnURef = 14,                             // 65550
    TokensBurnFailureNotEnoughBalance = 15,     // 65551
    MissingArgument0 = 16,                      // 65552
    MissingArgument1 = 17,                      // 65553
    MissingArgument2 = 18,                      // 65554
    MissingArgument3 = 19,                      // 65555
    MissingArgument4 = 20,                      // 65556
    MissingArgument5 = 21,                      // 65557
    InvalidArgument0 = 22,                      // 65558
    InvalidArgument1 = 23,                      // 65559
    InvalidArgument2 = 24,                      // 65560
    InvalidArgument3 = 25,                      // 65561
    InvalidArgument4 = 26,                      // 65562
    InvalidArgument5 = 27,                      // 65563
    UnsupportedNumberOfArguments = 28,          // 65564
    MissingERC20Contract = 29,                  // 65565
    MissingKey = 30,                            // 65566
    UnexpectedType = 31                         // 65567
}

impl Error {
    pub fn missing_argument(i: u32) -> Error {
        match i {
            0 => Error::MissingArgument0,
            1 => Error::MissingArgument1,
            2 => Error::MissingArgument2,
            3 => Error::MissingArgument3,
            4 => Error::MissingArgument4,
            5 => Error::MissingArgument5,
            _ => Error::UnsupportedNumberOfArguments,
        }
    }

    pub fn invalid_argument(i: u32) -> Error {
        match i {
            0 => Error::InvalidArgument0,
            1 => Error::InvalidArgument1,
            2 => Error::InvalidArgument2,
            3 => Error::InvalidArgument3,
            4 => Error::InvalidArgument4,
            5 => Error::InvalidArgument5,
            _ => Error::UnsupportedNumberOfArguments,
        }
    }
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

impl From<ERC20TransferError> for Error {
    fn from(error: ERC20TransferError) -> Error {
        match error {
            ERC20TransferError::NotEnoughBalance => Error::TransferFailureNotEnoughBalance
        }
    }
}

impl From<ERC20TransferFromError> for Error {
    fn from(error: ERC20TransferFromError) -> Error {
        match error {
            ERC20TransferFromError::TransferError(err) => Error::from(err),
            ERC20TransferFromError::NotEnoughAllowance => Error::TransferFromFailureNotEnoughAllowance
        }
    }
}
