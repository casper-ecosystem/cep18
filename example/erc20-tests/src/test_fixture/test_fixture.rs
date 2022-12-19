use super::utils::{execute_request, get_accounts, get_dictionary_value_from_key};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{
    DeployItemBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT, DEFAULT_PAYMENT,
    DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_erc20::constants as consts;
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, ContractHash, Key, RuntimeArgs, U256,
};
use std::path::PathBuf;

const CONTRACT_ERC20_TOKEN: &str = "erc20_token.wasm";
const CONTRACT_KEY_NAME: &str = "erc20_token_contract";

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestFixture {
    builder: InMemoryWasmTestBuilder,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl TestFixture {
    pub const TOKEN_NAME: &'static str = "Test ERC20";
    pub const TOKEN_SYMBOL: &'static str = "TERC";
    pub const TOKEN_DECIMALS: u8 = 8;
    const TOKEN_TOTAL_SUPPLY_AS_U64: u64 = 1000;

    pub fn token_total_supply() -> U256 {
        Self::TOKEN_TOTAL_SUPPLY_AS_U64.into()
    }

    pub fn install_contract() -> TestFixture {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();
        let (ali, bob, joe) = get_accounts(&mut builder);
        let session_code = PathBuf::from(CONTRACT_ERC20_TOKEN);
        let session_args = runtime_args! {
            consts::NAME_RUNTIME_ARG_NAME => TestFixture::TOKEN_NAME,
            consts::SYMBOL_RUNTIME_ARG_NAME => TestFixture::TOKEN_SYMBOL,
            consts::DECIMALS_RUNTIME_ARG_NAME => TestFixture::TOKEN_DECIMALS,
            consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME => TestFixture::token_total_supply()
        };
        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_session_code(session_code, session_args)
            .with_address(ali)
            .with_authorization_keys(&[ali])
            .build();
        execute_request(&mut builder, deploy_item);

        TestFixture {
            builder,
            ali,
            bob,
            joe,
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.builder
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        if let Ok(maybe_value) = self.builder.query(
            None,
            Key::from(self.ali),
            &[CONTRACT_KEY_NAME.to_string(), name.to_string()],
        ) {
            let value = maybe_value
                .as_cl_value()
                .expect("should be cl value.")
                .clone()
                .into_t()
                .unwrap_or_else(|_| panic!("{} is not expected type.", name));
            Some(value)
        } else {
            None
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_stored_session_hash(self.contract_hash(), method, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        execute_request(&mut self.builder, deploy_item);
    }

    pub fn token_name(&self) -> String {
        self.query_contract(consts::NAME_RUNTIME_ARG_NAME).unwrap()
    }

    pub fn token_symbol(&self) -> String {
        self.query_contract(consts::SYMBOL_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn token_decimals(&self) -> u8 {
        self.query_contract(consts::DECIMALS_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash().value());
        let value: Option<U256> = get_dictionary_value_from_key(
            &self.builder,
            &key,
            consts::BALANCES_KEY_NAME,
            &item_key,
        );
        value
    }

    pub fn allowance(&self, owner: Key, spender: Key) -> Option<U256> {
        let mut preimage = Vec::new();
        preimage.append(&mut owner.to_bytes().unwrap());
        preimage.append(&mut spender.to_bytes().unwrap());
        let key_bytes = blake2b256(&preimage);
        let allowance_item_key = hex::encode(&key_bytes);

        let key = Key::Hash(self.contract_hash().value());

        let value: Option<U256> = get_dictionary_value_from_key(
            &self.builder,
            &key,
            consts::ALLOWANCES_KEY_NAME,
            &allowance_item_key,
        );
        value
    }

    pub fn transfer(&mut self, recipient: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn approve(&mut self, spender: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::APPROVE_ENTRY_POINT_NAME,
            runtime_args! {
                consts::SPENDER_RUNTIME_ARG_NAME => spender,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                consts::OWNER_RUNTIME_ARG_NAME => owner,
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }
}
