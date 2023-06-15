use casper_types::{account::AccountHash, Key, PublicKey, SecretKey};
use once_cell::sync::Lazy;

pub const CEP18_CONTRACT_WASM: &str = "cep18.wasm";
pub const CEP18_TEST_CONTRACT_WASM: &str = "cep18_test_contract.wasm";
pub const NAME_KEY: &str = "name";
pub const SYMBOL_KEY: &str = "symbol";
pub const CEP18_TOKEN_CONTRACT_KEY: &str = "cep18_contract_hash_CasperTest";
pub const DECIMALS_KEY: &str = "decimals";
pub const TOTAL_SUPPLY_KEY: &str = "total_supply";
pub const BALANCES_KEY: &str = "balances";
pub const ALLOWANCES_KEY: &str = "allowances";
pub const OWNER: &str = "owner";
pub const AMOUNT: &str = "amount";

pub const ARG_NAME: &str = "name";
pub const ARG_SYMBOL: &str = "symbol";
pub const ARG_DECIMALS: &str = "decimals";
pub const ARG_TOTAL_SUPPLY: &str = "total_supply";

pub const _ERROR_INVALID_CONTEXT: u16 = 60000;
pub const ERROR_INSUFFICIENT_BALANCE: u16 = 60001;
pub const ERROR_INSUFFICIENT_ALLOWANCE: u16 = 60002;
pub const ERROR_OVERFLOW: u16 = 60003;

pub const TOKEN_NAME: &str = "CasperTest";
pub const TOKEN_SYMBOL: &str = "CSPRT";
pub const TOKEN_DECIMALS: u8 = 100;
pub const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;

pub const METHOD_TRANSFER: &str = "transfer";
pub const ARG_AMOUNT: &str = "amount";
pub const ARG_RECIPIENT: &str = "recipient";

pub const METHOD_APPROVE: &str = "approve";
pub const ARG_OWNER: &str = "owner";
pub const ARG_SPENDER: &str = "spender";

pub const METHOD_TRANSFER_FROM: &str = "transfer_from";

pub const CHECK_TOTAL_SUPPLY_ENTRYPOINT: &str = "check_total_supply";
pub const CHECK_BALANCE_OF_ENTRYPOINT: &str = "check_balance_of";
pub const CHECK_ALLOWANCE_OF_ENTRYPOINT: &str = "check_allowance_of";
pub const ARG_TOKEN_CONTRACT: &str = "token_contract";
pub const ARG_ADDRESS: &str = "address";
pub const RESULT_KEY: &str = "result";
pub const CEP18_TEST_CONTRACT_KEY: &str = "cep18_test_contract";

pub static ACCOUNT_1_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes([221u8; 32]).unwrap());
pub static ACCOUNT_1_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_1_SECRET_KEY));
pub static ACCOUNT_1_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_1_PUBLIC_KEY.to_account_hash());

pub static ACCOUNT_2_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes([212u8; 32]).unwrap());
pub static ACCOUNT_2_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_2_SECRET_KEY));
pub static ACCOUNT_2_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_2_PUBLIC_KEY.to_account_hash());

pub const TRANSFER_AMOUNT_1: u64 = 200_001;
pub const TRANSFER_AMOUNT_2: u64 = 19_999;
pub const ALLOWANCE_AMOUNT_1: u64 = 456_789;
pub const ALLOWANCE_AMOUNT_2: u64 = 87_654;

pub const METHOD_TRANSFER_AS_STORED_CONTRACT: &str = "transfer_as_stored_contract";
pub const METHOD_APPROVE_AS_STORED_CONTRACT: &str = "approve_as_stored_contract";
pub const METHOD_FROM_AS_STORED_CONTRACT: &str = "transfer_from_as_stored_contract";

pub const TOKEN_OWNER_ADDRESS_1: Key = Key::Account(AccountHash::new([42; 32]));
pub const TOKEN_OWNER_AMOUNT_1: u64 = 1_000_000;
pub const TOKEN_OWNER_ADDRESS_2: Key = Key::Hash([42; 32]);
pub const TOKEN_OWNER_AMOUNT_2: u64 = 2_000_000;

pub const METHOD_MINT: &str = "mint";
pub const METHOD_BURN: &str = "burn";
pub const DECREASE_ALLOWANCE: &str = "decrease_allowance";
pub const INCREASE_ALLOWANCE: &str = "increase_allowance";
pub const ENABLE_MINT_BURN: &str = "enable_mint_burn";
pub const ADMIN_LIST: &str = "admin_list";
pub const MINTER_LIST: &str = "minter_list";
pub const NONE_LIST: &str = "none_list";
pub const CHANGE_SECURITY: &str = "change_security";
