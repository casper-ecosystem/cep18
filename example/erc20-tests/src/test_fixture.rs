use std::path::PathBuf;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

use casper_engine_test_support::{InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST};
use casper_erc20::constants as consts;
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256
};

use crate::utils::{fund_account, DeploySource, deploy, query, query_dictionary_item};

const CONTRACT_ERC20_TOKEN: &str = "erc20_token.wasm";
const CONTRACT_KEY_NAME: &str = "erc20_token_contract";

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

pub struct TestFixture {
    builder: InMemoryWasmTestBuilder,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
    pub contract_hash: ContractHash,
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
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap().to_account_hash();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap().to_account_hash();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap().to_account_hash();

        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();
        builder
            .exec(fund_account(&ali))
            .expect_success()
            .commit();
        builder
            .exec(fund_account(&bob))
            .expect_success()
            .commit();

        let session_code = PathBuf::from(CONTRACT_ERC20_TOKEN);
        let session_args = runtime_args! {
            consts::NAME_RUNTIME_ARG_NAME => TestFixture::TOKEN_NAME,
            consts::SYMBOL_RUNTIME_ARG_NAME => TestFixture::TOKEN_SYMBOL,
            consts::DECIMALS_RUNTIME_ARG_NAME => TestFixture::TOKEN_DECIMALS,
            consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME => TestFixture::token_total_supply()
        };

        deploy(
            &mut builder,
            &ali,
            &DeploySource::Code(session_code),
            session_args,
            true,
            None,
        );

        let contract_hash = builder
            .get_account(ali)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into();

        TestFixture {
            builder,
            ali,
            bob,
            joe,
            contract_hash
        }
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> T {
        query(
            &self.builder,
            Key::Account(self.ali),
            &[CONTRACT_KEY_NAME.to_string(), name.to_string()],
        )
    }

    fn call(&mut self, sender: AccountHash, method: &str, args: RuntimeArgs) {
        deploy(
            &mut self.builder,
            &sender,
            &DeploySource::ByContractHash { hash: self.contract_hash, entry_point: method.to_string() },
            args,
            true,
            None,
        );
    }

    pub fn token_name(&self) -> String {
        self.query_contract(consts::NAME_RUNTIME_ARG_NAME)
    }

    pub fn token_symbol(&self) -> String {
        self.query_contract(consts::SYMBOL_RUNTIME_ARG_NAME)
    }

    pub fn token_decimals(&self) -> u8 {
        self.query_contract(consts::DECIMALS_RUNTIME_ARG_NAME)
    }

    // TODO: this breaks, why?
    pub fn balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash.value());
        match query_dictionary_item(
            &self.builder, key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
            {
                Ok(value)=>Some(
                    value
                        .as_cl_value()
                        .expect("should be cl value. (balance_of)")
                        .clone()
                        .into_t()
                        .expect("cannot convert into type")
                    ),
                Err(_)=> None
            }
    }

    // pub fn balance_of(&self, account: Key) -> Option<U256> {
    //     let item_key = base64::encode(&account.to_bytes().unwrap());

    //     let key = Key::Hash(self.contract_hash().value());
    //     let value = self
    //         .context
    //         .query_dictionary_item(key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
    //         .ok()?;

    //     Some(value.into_t::<U256>().unwrap())
    // }

    pub fn allowance(&self, owner: Key, spender: Key) -> Option<U256> {
        let mut preimage = Vec::new();
        preimage.append(&mut owner.to_bytes().unwrap());
        preimage.append(&mut spender.to_bytes().unwrap());
        let key_bytes = blake2b256(&preimage);
        let allowance_item_key = hex::encode(&key_bytes);

        let key = Key::Hash(self.contract_hash.value());

        match
            query_dictionary_item(
                &self.builder,
                key,
                Some(consts::ALLOWANCES_KEY_NAME.to_string()),
                allowance_item_key,
            ).expect("should be stored value. (allowance)")
            .as_cl_value()
            .expect("should be cl value. (allowance)")
            .clone()
            .into_t(){
                Ok(value)=>Some(value),
                Err(_)=> None
            }
    }

    pub fn transfer(&mut self, recipient: Key, amount: U256, sender: AccountHash) {
        self.call(
            sender,
            consts::TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn approve(&mut self, spender: Key, amount: U256, sender: AccountHash) {
        self.call(
            sender,
            consts::APPROVE_ENTRY_POINT_NAME,
            runtime_args! {
                consts::SPENDER_RUNTIME_ARG_NAME => spender,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256, sender: AccountHash) {
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
