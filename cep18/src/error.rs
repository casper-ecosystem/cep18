//! Error handling on the casper platform.
use casper_types::ApiError;

/// Errors which can be returned by the library.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// Where a smart contract consuming this library needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Cep18Error {
    /// CEP18 contract called from within an invalid context.
    InvalidContext = 60000,
    /// Spender does not have enough balance.
    InsufficientBalance = 60001,
    /// Spender does not have enough allowance approved.
    InsufficientAllowance = 60002,
    /// Operation would cause an integer overflow.
    Overflow = 60003,
    PackageHashMissing = 60004,
    PackageHashNotPackage = 60005,
    InvalidEventsMode = 60006,
    MissingEventsMode = 60007,
    Phantom = 60008,
    FailedToGetArgBytes = 60009,
    InsufficientRights = 60010,
    InvalidAdminList = 60011,
    InvalidMinterList = 60012,
    InvalidBurnerList = 60013,
    InvalidMintAndBurnList = 60014,
    InvalidNoneList = 60015,
    InvalidEnableMBFlag = 60016,
    AlreadyInitialized = 60017,
}

impl From<Cep18Error> for ApiError {
    fn from(error: Cep18Error) -> Self {
        ApiError::User(error as u16)
    }
}
