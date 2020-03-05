use casperlabs_contract::args_parser::ArgsParser;
use casperlabs_engine_test_support::{
    Code, Hash, PublicKey, SessionBuilder, TestContext, TestContextBuilder,
};
use casperlabs_types::{bytesrepr::FromBytes, CLTyped, Key, U512};

const ERC20_WASM: &str = "contract.wasm";
pub const ERC20_INIT_BALANCE: u64 = 10000;

pub mod account {
    use super::PublicKey;
    pub const ALI: PublicKey = PublicKey::ed25519_from([1u8; 32]);
    pub const BOB: PublicKey = PublicKey::ed25519_from([2u8; 32]);
    pub const JOE: PublicKey = PublicKey::ed25519_from([3u8; 32]);
}

mod method {
    pub const DEPLOY: &str = "deploy";
    pub const TRANSFER: &str = "transfer";
    pub const TRANSFER_FROM: &str = "transfer_from";
    pub const APPROVE: &str = "approve";
}

mod key {
    pub const ERC20_PROXY: &str = "erc20_proxy";
    pub const ERC20: &str = "erc20";
    pub const TOTAL_SUPPLY: &str = "total_supply";
}

pub struct Sender(pub PublicKey);

pub struct ERC20Contract {
    pub context: TestContext,
    pub token_hash: Hash,
    pub proxy_hash: Hash,
}

impl ERC20Contract {
    pub fn deployed() -> Self {
        // Init context.
        let clx_init_balance = U512::from(10_000_000_000u64);
        let mut context = TestContextBuilder::new()
            .with_account(account::ALI, clx_init_balance)
            .with_account(account::BOB, clx_init_balance)
            .with_account(account::JOE, clx_init_balance)
            .build();
        // Deploy contract.
        let code = Code::from(ERC20_WASM);
        let args = (method::DEPLOY, U512::from(ERC20_INIT_BALANCE));
        let session = SessionBuilder::new(code, args)
            .with_address(account::ALI)
            .with_authorization_keys(&[account::ALI])
            .build();
        context.run(session);
        // Read hashes.
        let token_hash = Self::contract_hash(&context, key::ERC20);
        let proxy_hash = Self::contract_hash(&context, key::ERC20_PROXY);
        Self {
            context,
            token_hash,
            proxy_hash,
        }
    }

    pub fn transfer(&mut self, reciever: PublicKey, amount: u64, sender: Sender) {
        self.call_proxy(
            sender,
            (
                (method::TRANSFER, self.token_hash),
                reciever,
                U512::from(amount),
            ),
        )
    }

    pub fn approve(&mut self, spender: PublicKey, amount: u64, sender: Sender) {
        self.call_proxy(
            sender,
            (
                (method::APPROVE, self.token_hash),
                spender,
                U512::from(amount),
            ),
        )
    }

    pub fn transfer_from(
        &mut self,
        owner: PublicKey,
        reciever: PublicKey,
        amount: u64,
        sender: Sender,
    ) {
        self.call_proxy(
            sender,
            (
                (method::TRANSFER_FROM, self.token_hash),
                owner,
                reciever,
                U512::from(amount),
            ),
        )
    }

    pub fn balance_of(&self, account: PublicKey) -> u64 {
        let balance: Option<U512> = self.query_contract(account.to_string());
        balance.unwrap_or_else(U512::zero).as_u64()
    }

    pub fn allowance(&self, owner: PublicKey, spender: PublicKey) -> u64 {
        let allowance: Option<U512> = self.query_contract(format!("{}{}", owner, spender));
        allowance.unwrap_or_else(U512::zero).as_u64()
    }

    pub fn total_supply(&self) -> u64 {
        let balance: Option<U512> = self.query_contract(key::TOTAL_SUPPLY.to_string());
        balance.unwrap().as_u64()
    }

    fn call_proxy(&mut self, sender: Sender, args: impl ArgsParser) {
        let Sender(address) = sender;
        let code = Code::Hash(self.proxy_hash);
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    fn contract_hash(context: &TestContext, name: &str) -> Hash {
        let contract_ref: Key = context
            .query(account::ALI, &[name])
            .unwrap_or_else(|_| panic!("{} contract not found.", name))
            .into_t()
            .unwrap_or_else(|_| panic!("{} is not a type Contract.", name));
        contract_ref
            .into_hash()
            .unwrap_or_else(|| panic!("{} is not a type Hash.", name))
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: String) -> Option<T> {
        match self.context.query(account::ALI, &[key::ERC20, &name]) {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }
}
