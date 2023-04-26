use casper_types::{SecretKey, PublicKey, account::AccountHash, Key};
use once_cell::sync::Lazy;


pub (crate) const CEP18_CONTRACT_WASM: &str = "cep18.wasm";
pub (crate) const CEP18_TEST_HASH: &str = "test_contract_package_hash";
pub (crate) const CEP18_TEST_CONTARCT_WASM: &str = "cep18_test_contract.wasm";
pub (crate) const CEP18_TEST_WASM: &str = "cep18_test.wasm";
pub (crate) const NAME_KEY: &str = "name";
pub (crate) const SYMBOL_KEY: &str = "symbol";
pub (crate) const CEP18_TOKEN_CONTRACT_KEY: &str = "cep18_contract_hash_CasperTest";
pub (crate) const DECIMALS_KEY: &str = "decimals";
pub (crate) const TOTAL_SUPPLY_KEY: &str = "total_supply";
pub (crate) const BALANCES_KEY: &str = "balances";
pub (crate) const ALLOWANCES_KEY: &str = "allowances";

pub (crate) const ARG_NAME: &str = "name";
pub (crate) const ARG_SYMBOL: &str = "symbol";
pub (crate) const ARG_DECIMALS: &str = "decimals";
pub (crate) const ARG_TOTAL_SUPPLY: &str = "total_supply";

pub (crate) const TEST_CONTRACT_KEY: &str = "test_contract_hash";

pub (crate) const _ERROR_INVALID_CONTEXT: u16 = u16::MAX;
pub (crate) const ERROR_INSUFFICIENT_BALANCE: u16 = u16::MAX - 1;
pub (crate) const ERROR_INSUFFICIENT_ALLOWANCE: u16 = u16::MAX - 2;
pub (crate) const ERROR_OVERFLOW: u16 = u16::MAX - 3;

pub (crate) const TOKEN_NAME: &str = "CasperTest";
pub (crate) const TOKEN_SYMBOL: &str = "CSPRT";
pub (crate) const TOKEN_DECIMALS: u8 = 100;
pub (crate) const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;

pub (crate) const METHOD_TRANSFER: &str = "transfer";
pub (crate) const ARG_AMOUNT: &str = "amount";
pub (crate) const ARG_RECIPIENT: &str = "recipient";

pub (crate) const METHOD_APPROVE: &str = "approve";
pub (crate) const ARG_OWNER: &str = "owner";
pub (crate) const ARG_SPENDER: &str = "spender";

pub (crate) const METHOD_TRANSFER_FROM: &str = "transfer_from";

pub (crate) const CHECK_TOTAL_SUPPLY_ENTRYPOINT: &str = "check_total_supply";
pub (crate) const CHECK_BALANCE_OF_ENTRYPOINT: &str = "check_balance_of";
pub (crate) const CHECK_ALLOWANCE_OF_ENTRYPOINT: &str = "check_allowance_of";
pub (crate) const ARG_TOKEN_CONTRACT: &str = "token_contract";
pub (crate) const ARG_ADDRESS: &str = "address";
pub (crate) const RESULT_KEY: &str = "result";
pub (crate) const CEP18_TEST_CONTRACT_KEY: &str = "cep18_test_contract";

pub (crate) static ACCOUNT_1_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes([221u8; 32]).unwrap());
pub (crate) static ACCOUNT_1_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_1_SECRET_KEY));
pub (crate) static ACCOUNT_1_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_1_PUBLIC_KEY.to_account_hash());

pub (crate) static ACCOUNT_2_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes([212u8; 32]).unwrap());
pub (crate) static ACCOUNT_2_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_2_SECRET_KEY));
pub (crate) static ACCOUNT_2_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_2_PUBLIC_KEY.to_account_hash());

pub (crate) const TRANSFER_AMOUNT_1: u64 = 200_001;
pub (crate) const TRANSFER_AMOUNT_2: u64 = 19_999;
pub (crate) const ALLOWANCE_AMOUNT_1: u64 = 456_789;
pub (crate) const ALLOWANCE_AMOUNT_2: u64 = 87_654;

pub (crate) const METHOD_TRANSFER_AS_STORED_CONTRACT: &str = "transfer_as_stored_contract";
pub (crate) const METHOD_APPROVE_AS_STORED_CONTRACT: &str = "approve_as_stored_contract";
pub (crate) const METHOD_FROM_AS_STORED_CONTRACT: &str = "transfer_from_as_stored_contract";

pub (crate) const TOKEN_OWNER_ADDRESS_1: Key = Key::Account(AccountHash::new([42; 32]));
pub (crate) const TOKEN_OWNER_AMOUNT_1: u64 = 1_000_000;
pub (crate) const TOKEN_OWNER_ADDRESS_2: Key = Key::Hash([42; 32]);
pub (crate) const TOKEN_OWNER_AMOUNT_2: u64 = 2_000_000;

pub (crate) const METHOD_MINT: &str = "mint";
pub (crate) const METHOD_BURN: &str = "burn";