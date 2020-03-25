#[derive(PartialEq, Debug)]
pub enum ERC20TransferError {
    NotEnoughBalance,
}

#[derive(PartialEq, Debug)]
pub enum ERC20TransferFromError {
    TransferError(ERC20TransferError),
    NotEnoughAllowance,
}

impl From<ERC20TransferError> for ERC20TransferFromError {
    fn from(error: ERC20TransferError) -> ERC20TransferFromError {
        ERC20TransferFromError::TransferError(error)
    }
}
