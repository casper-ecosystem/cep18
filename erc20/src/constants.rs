//! Constants used by the ERC20 contract.

/// Name of named-key for `name`.
pub const NAME_KEY_NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL_KEY_NAME: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS_KEY_NAME: &str = "decimals";
/// Name of named-key for `contract`
pub const ERC20_TOKEN_CONTRACT_KEY_NAME: &str = "erc20_token_contract";
/// Name of dictionary-key for `balances`
pub const BALANCES_KEY_NAME: &str = "balances";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES_KEY_NAME: &str = "allowances";
/// Name of named-key for `total_supply`
pub const TOTAL_SUPPLY_KEY_NAME: &str = "total_supply";

//////////// stakes
/// Name of dictionary-key for `stakers`
pub const STAKERS_KEY_NAME: &str = "stakers";
/// Name of dictionary-key for `stakes`
pub const STAKES_KEY_NAME: &str = "stakes";
/// Name of dictionary-key `rewards`
pub const REWARDS_KEY_NAME: &str = "rewards";

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

/// Name of `address` runtime argument.
pub const ADDRESS_RUNTIME_ARG_NAME: &str = "address";
/// Name of `owner` runtime argument.
pub const OWNER_RUNTIME_ARG_NAME: &str = "owner";
/// Name of `spender` runtime argument.
pub const SPENDER_RUNTIME_ARG_NAME: &str = "spender";
/// Name of `amount` runtime argument.
pub const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
/// Name of `recipient` runtime argument.
pub const RECIPIENT_RUNTIME_ARG_NAME: &str = "recipient";
/// Name of `name` runtime argument.
pub const NAME_RUNTIME_ARG_NAME: &str = "name";
/// Name of `symbol` runtime argument.
pub const SYMBOL_RUNTIME_ARG_NAME: &str = "symbol";
/// Name of `decimals` runtime argument.
pub const DECIMALS_RUNTIME_ARG_NAME: &str = "decimals";
/// Name of `total_supply` runtime argument.
pub const TOTAL_SUPPLY_RUNTIME_ARG_NAME: &str = "total_supply";
