use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casperlabs_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, RuntimeArgs, U256, U512,
};
use casperlabs_types::PublicKey;
pub mod account {
    use super::*;
    pub const ALI: PublicKey = PublicKey::ed25519([3u8; 32]).unwrap();
    pub const BOB: AccountHash = AccountHash::new([8u8; 32]);
    pub const JOE: AccountHash = AccountHash::new([9u8; 32]);
}

pub mod token_cfg {
    use super::*;
    pub const NAME: &str = "ERC20";
    pub const SYMBOL: &str = "STX";
    pub const DECIMALS: u8 = 18;
    pub fn total_supply() -> U256 {
        1_000.into()
    }
}

pub struct Sender(pub AccountHash);

pub struct Token {
    context: TestContext,
}

impl Token {
    pub fn deployed() -> Token {

        let mut context = TestContextBuilder::new()
            .with_public_key(account::ALI, account::ALI.to_account_hash(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from("contract.wasm");
        let session_args = runtime_args! {
            "tokenName" => token_cfg::NAME,
            "tokenSymbol" => token_cfg::SYMBOL,
            "tokenTotalSupply" => token_cfg::total_supply()
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account::ALI.to_account_hash())
            .with_authorization_keys(&[account::ALI.to_account_hash()])
            .build();
        context.run(session);
        Token { context }
    }

    fn contract_hash(&self) -> Hash {
        self.context
            .query(account::ALI.to_account_hash(), &[format!("{}_hash", token_cfg::NAME)])
            .unwrap_or_else(|_| panic!("{} contract not found", token_cfg::NAME))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", token_cfg::NAME))
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(account::ALI.to_account_hash(), &[token_cfg::NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn name(&self) -> String {
        self.query_contract("_name").unwrap()
    }

    pub fn symbol(&self) -> String {
        self.query_contract("_symbol").unwrap()
    }

    pub fn decimals(&self) -> u8 {
        self.query_contract("_decimals").unwrap()
    }

    pub fn balance_of(&self, account: AccountHash) -> U256 {
        let key = format!("_balances_{}", account);
        self.query_contract(&key).unwrap_or_default()
    }

    pub fn allowance(&self, owner: AccountHash, spender: AccountHash) -> U256 {
        let key = format!("_allowances_{}_{}", owner, spender);
        self.query_contract(&key).unwrap_or_default()
    }

    pub fn transfer(&mut self, recipient: AccountHash, amount: U256, sender: Sender) {
        self.call(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    pub fn approve(&mut self, spender: AccountHash, amount: U256, sender: Sender) {
        self.call(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount
            },
        );
    }

    pub fn transfer_from(
        &mut self,
        owner: AccountHash,
        recipient: AccountHash,
        amount: U256,
        sender: Sender,
    ) {
        self.call(
            sender,
            "transferFrom",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }
}
