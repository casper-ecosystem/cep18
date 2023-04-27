//! Constants used by the CEP18 contract.

/// Name of named-key for `name`.
pub const NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS: &str = "decimals";
/// Name of named-key for `contract`
pub const CEP18_TOKEN_CONTRACT_KEY_NAME: &str = "cep18_token_contract";
/// Name of dictionary-key for `balances`
pub const BALANCES: &str = "balances";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES: &str = "allowances";
/// Name of named-key for `total_supply`
pub const TOTAL_SUPPLY: &str = "total_supply";

pub const HASH_KEY_NAME_PREFIX: &str = "cep18_contract_package_";
pub const ACCESS_KEY_NAME_PREFIX: &str = "cep18_contract_package_access_";
pub const CONTRACT_NAME_PREFIX: &str = "cep18_contract_hash_";
pub const CONTRACT_VERSION_PREFIX: &str = "cep18_contract_version_";

/// Name of `name` entry point.
pub const NAME_ENTRY_POINT_NAME: &str = "name";
/// Name of `symbol` entry point.
pub const SYMBOL_ENTRY_POINT_NAME: &str = "symbol";
/// Name of `decimals` entry point.
pub const DECIMALS_ENTRY_POINT_NAME: &str = "decimals";
/// Name of `balance_of` entry point.
pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";
/// Name of `transfer` entry point.
pub const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
/// Name of `approve` entry point.
pub const APPROVE_ENTRY_POINT_NAME: &str = "approve";
/// Name of `allowance` entry point.
pub const ALLOWANCE_ENTRY_POINT_NAME: &str = "allowance";
/// Name of `transfer_from` entry point.
pub const TRANSFER_FROM_ENTRY_POINT_NAME: &str = "transfer_from";
/// Name of `total_supply` entry point.
pub const TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "total_supply";
/// Name of `transfer_from` entry point.
pub const MINT_ENTRY_POINT_NAME: &str = "mint";
/// Name of `burn` entry point.
pub const BURN_ENTRY_POINT_NAME: &str = "burn";
/// Name of `init` entry point.
pub const ENTRY_POINT_INIT: &str = "init";

/// Name of `address` runtime argument.
pub const ADDRESS: &str = "address";
/// Name of `owner` runtime argument.
pub const OWNER: &str = "owner";
/// Name of `spender` runtime argument.
pub const SPENDER: &str = "spender";
/// Name of `amount` runtime argument.
pub const AMOUNT: &str = "amount";
/// Name of `recipient` runtime argument.
pub const RECIPIENT: &str = "recipient";
pub const PACKAGE_HASH: &str = "package_hash";
