use super::constants::{
    CEP18_CONTRACT_WASM, CEP18_TEST_CONTRACT_WASM, CEP18_TEST_TOKEN_CONTRACT_NAME, TOKEN_DECIMALS,
    TOKEN_NAME, TOKEN_SYMBOL, TOKEN_TOTAL_SUPPLY,
};
use crate::utility::constants::{
    AMOUNT_ALLOWANCE_1, AMOUNT_ALLOWANCE_2, AMOUNT_TRANSFER_1, AMOUNT_TRANSFER_2,
};
use casper_engine_test_support::{
    utils::create_run_genesis_request, ExecuteRequest, ExecuteRequestBuilder, LmdbWasmTestBuilder,
    DEFAULT_ACCOUNTS, DEFAULT_ACCOUNT_ADDR,
};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, AddressableEntityHash, CLTyped,
    EntityAddr, Key, PackageHash, PublicKey, RuntimeArgs, U256,
};
use cep18::{
    constants::{
        ARG_ADDRESS, ARG_AMOUNT, ARG_DECIMALS, ARG_EVENTS_MODE, ARG_NAME, ARG_OWNER, ARG_RECIPIENT,
        ARG_SPENDER, ARG_SYMBOL, ARG_TOTAL_SUPPLY, ENTRY_POINT_APPROVE, ENTRY_POINT_TRANSFER,
    },
    modalities::EventsMode,
};
use cep18_test_contract::constants::{
    ARG_TOKEN_CONTRACT, CEP18_TEST_CONTRACT_PACKAGE_NAME, ENTRY_POINT_APPROVE_AS_STORED_CONTRACT,
    ENTRY_POINT_CHECK_ALLOWANCE_OF, ENTRY_POINT_CHECK_BALANCE_OF, ENTRY_POINT_CHECK_TOTAL_SUPPLY,
    ENTRY_POINT_TRANSFER_AS_STORED_CONTRACT, RESULT_KEY,
};

/// Converts hash addr of Account into Hash, and Hash into Account
///
/// This is useful for making sure CEP18 library respects different variants of Key when storing
/// balances.
pub(crate) fn invert_cep18_address(address: Key) -> Key {
    match address {
        Key::Account(account_hash) => Key::Hash(account_hash.value()),
        Key::Hash(contract_hash) => Key::Account(AccountHash::new(contract_hash)),
        Key::AddressableEntity(entity_addr) => match entity_addr {
            EntityAddr::System(_) => panic!("Unsupported Key variant"),
            EntityAddr::Account(account) => Key::SmartContract(account),
            EntityAddr::SmartContract(_) => panic!("Unsupported Key variant"),
        },
        Key::SmartContract(contract_package_hash) => {
            Key::AddressableEntity(EntityAddr::Account(contract_package_hash))
        }
        _ => panic!("Unsupported Key variant"),
    }
}

#[derive(Copy, Clone)]
pub(crate) struct TestContext {
    pub(crate) cep18_contract_hash: AddressableEntityHash,
    pub(crate) cep18_test_contract_package: PackageHash,
}

pub(crate) fn setup() -> (LmdbWasmTestBuilder, TestContext) {
    setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ARG_EVENTS_MODE => EventsMode::Native as u8
    })
}

pub(crate) fn setup_with_args(install_args: RuntimeArgs) -> (LmdbWasmTestBuilder, TestContext) {
    let mut builder = LmdbWasmTestBuilder::default();
    builder
        .run_genesis(create_run_genesis_request(DEFAULT_ACCOUNTS.to_vec()))
        .commit();

    let install_request_1 =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CEP18_CONTRACT_WASM, install_args)
            .build();

    let install_request_2 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_TEST_CONTRACT_WASM,
        RuntimeArgs::default(),
    )
    .build();

    builder.exec(install_request_1).expect_success().commit();
    builder.exec(install_request_2).expect_success().commit();

    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .unwrap();
    let account_named_keys = account.named_keys();

    let cep18_contract_hash = account_named_keys
        .get(CEP18_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_entity_hash())
        .expect("should have contract hash");

    let cep18_test_contract_package = account_named_keys
        .get(CEP18_TEST_CONTRACT_PACKAGE_NAME)
        .and_then(|key| key.into_package_hash())
        .expect("should have package hash");

    let test_context = TestContext {
        cep18_contract_hash,
        cep18_test_contract_package,
    };

    (builder, test_context)
}

pub(crate) fn get_test_account(ending_string_index: &str) -> (Key, AccountHash, PublicKey) {
    let index = ending_string_index
        .chars()
        .next_back()
        .unwrap()
        .to_digit(10)
        .unwrap_or_default() as usize;

    let accounts = if let Some(account) = DEFAULT_ACCOUNTS.clone().get(index) {
        let public_key = account.public_key().clone();
        let account_hash = public_key.to_account_hash();
        let entity_addr = Key::Account(account_hash);
        Some((entity_addr, account_hash, public_key))
    } else {
        None
    };

    match accounts {
        Some(account) => account,
        None => {
            panic!("No account found for index {index}");
        }
    }
}

pub(crate) fn cep18_check_total_supply(
    builder: &mut LmdbWasmTestBuilder,
    cep18_contract_hash: &AddressableEntityHash,
) -> U256 {
    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep18_test_contract_package = account
        .named_keys()
        .get(CEP18_TEST_CONTRACT_PACKAGE_NAME)
        .and_then(|key| key.into_package_hash())
        .expect("should have test contract hash");

    let check_total_supply_args = runtime_args! {
        ARG_TOKEN_CONTRACT => Key::Hash(cep18_contract_hash.value()),
    };

    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_test_contract_package,
        None,
        ENTRY_POINT_CHECK_TOTAL_SUPPLY,
        check_total_supply_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();

    get_test_result(builder, cep18_test_contract_package)
}

pub(crate) fn get_test_result<T: FromBytes + CLTyped>(
    builder: &mut LmdbWasmTestBuilder,
    cep18_test_contract_package: PackageHash,
) -> T {
    let contract_package = builder
        .get_package(cep18_test_contract_package)
        .expect("should have contract package");
    let enabled_versions = contract_package.enabled_versions();

    let contract_hash = enabled_versions
        .contract_hashes()
        .last()
        .expect("should have latest version");
    let contract_entity_addr = EntityAddr::new_smart_contract(contract_hash.value());
    builder.get_value(contract_entity_addr, RESULT_KEY)
}

pub(crate) fn cep18_check_balance_of(
    builder: &mut LmdbWasmTestBuilder,
    cep18_contract_hash: &AddressableEntityHash,
    address: Key,
) -> U256 {
    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep18_test_contract_package = account
        .named_keys()
        .get(CEP18_TEST_CONTRACT_PACKAGE_NAME)
        .and_then(|key| key.into_package_hash())
        .expect("should have test contract package hash");

    let check_balance_args = runtime_args! {
        ARG_TOKEN_CONTRACT => Key::Hash(cep18_contract_hash.value()),
        ARG_ADDRESS => address,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_test_contract_package,
        None,
        ENTRY_POINT_CHECK_BALANCE_OF,
        check_balance_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();

    get_test_result(builder, cep18_test_contract_package)
}

pub(crate) fn cep18_check_allowance_of(
    builder: &mut LmdbWasmTestBuilder,
    owner: Key,
    spender: Key,
) -> U256 {
    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");
    let cep18_contract_hash = account
        .named_keys()
        .get(CEP18_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_entity_hash())
        .expect("should have test contract hash");
    let cep18_test_contract_package = account
        .named_keys()
        .get(CEP18_TEST_CONTRACT_PACKAGE_NAME)
        .and_then(|key| key.into_package_hash())
        .expect("should have test contract hash");

    let check_balance_args = runtime_args! {
        ARG_TOKEN_CONTRACT => Key::Hash(cep18_contract_hash.value()),
        ARG_OWNER => owner,
        ARG_SPENDER => spender,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_test_contract_package,
        None,
        ENTRY_POINT_CHECK_ALLOWANCE_OF,
        check_balance_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();

    get_test_result(builder, cep18_test_contract_package)
}

pub(crate) fn test_cep18_transfer(
    builder: &mut LmdbWasmTestBuilder,
    test_context: &TestContext,
    sender1: Key,
    recipient1: Key,
    sender2: Key,
    recipient2: Key,
) {
    let TestContext {
        cep18_contract_hash,
        ..
    } = test_context;

    let transfer_amount_1 = U256::from(AMOUNT_TRANSFER_1);
    let transfer_amount_2 = U256::from(AMOUNT_TRANSFER_2);

    let sender_balance_before = cep18_check_balance_of(builder, cep18_contract_hash, sender1);
    assert_ne!(sender_balance_before, U256::zero());

    let account_1_balance_before = cep18_check_balance_of(builder, cep18_contract_hash, recipient1);
    assert_eq!(account_1_balance_before, U256::zero());

    let account_2_balance_before = cep18_check_balance_of(builder, cep18_contract_hash, recipient2);
    assert_eq!(account_2_balance_before, U256::zero());

    let token_transfer_request_1 =
        make_cep18_transfer_request(sender1, cep18_contract_hash, recipient1, transfer_amount_1);

    builder
        .exec(token_transfer_request_1)
        .expect_success()
        .commit();

    let account_1_balance_after = cep18_check_balance_of(builder, cep18_contract_hash, recipient1);
    assert_eq!(account_1_balance_after, transfer_amount_1);

    let account_1_balance_before = account_1_balance_after;

    let sender_balance_after = cep18_check_balance_of(builder, cep18_contract_hash, sender1);
    assert_eq!(
        sender_balance_after,
        sender_balance_before - transfer_amount_1
    );
    let sender_balance_before = sender_balance_after;

    let token_transfer_request_2 =
        make_cep18_transfer_request(sender2, cep18_contract_hash, recipient2, transfer_amount_2);

    builder
        .exec(token_transfer_request_2)
        .expect_success()
        .commit();

    let sender_balance_after = cep18_check_balance_of(builder, cep18_contract_hash, sender1);
    assert_eq!(sender_balance_after, sender_balance_before);

    let account_1_balance_after = cep18_check_balance_of(builder, cep18_contract_hash, recipient1);
    assert!(account_1_balance_after < account_1_balance_before);
    assert_eq!(
        account_1_balance_after,
        transfer_amount_1 - transfer_amount_2
    );

    let account_2_balance_after = cep18_check_balance_of(builder, cep18_contract_hash, recipient2);
    assert_eq!(account_2_balance_after, transfer_amount_2);
}

pub(crate) fn make_cep18_transfer_request(
    sender: Key,
    cep18_contract_hash: &AddressableEntityHash,
    recipient: Key,
    amount: U256,
) -> ExecuteRequest {
    match sender {
        Key::Account(sender) => ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            AddressableEntityHash::new(cep18_contract_hash.value()),
            ENTRY_POINT_TRANSFER,
            runtime_args! {
                ARG_AMOUNT => amount,
                ARG_RECIPIENT => recipient,
            },
        )
        .build(),
        Key::Hash(contract_package_hash) => ExecuteRequestBuilder::versioned_contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            PackageHash::new(contract_package_hash),
            None,
            ENTRY_POINT_TRANSFER_AS_STORED_CONTRACT,
            runtime_args! {
                ARG_TOKEN_CONTRACT => Key::Hash(cep18_contract_hash.value()),
                ARG_AMOUNT => amount,
                ARG_RECIPIENT => recipient,
            },
        )
        .build(),
        Key::AddressableEntity(entity_addr) => match entity_addr {
            EntityAddr::System(_) => panic!("Not a use case"),
            EntityAddr::Account(account_addr) => ExecuteRequestBuilder::contract_call_by_hash(
                AccountHash::new(account_addr),
                AddressableEntityHash::new(cep18_contract_hash.value()),
                ENTRY_POINT_TRANSFER,
                runtime_args! {
                    ARG_AMOUNT => amount,
                    ARG_RECIPIENT => recipient,
                },
            )
            .build(),
            EntityAddr::SmartContract(_contract_hash) => panic!("invalid variant"),
        },
        Key::SmartContract(package_hash) => ExecuteRequestBuilder::versioned_contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            PackageHash::new(package_hash),
            None,
            ENTRY_POINT_TRANSFER_AS_STORED_CONTRACT,
            runtime_args! {
                ARG_TOKEN_CONTRACT => Key::Hash(cep18_contract_hash.value()),
                ARG_AMOUNT => amount,
                ARG_RECIPIENT => recipient,
            },
        )
        .build(),
        _ => panic!("Unknown variant"),
    }
}

pub(crate) fn make_cep18_approve_request(
    sender: Key,
    cep18_contract_hash: &AddressableEntityHash,
    spender: Key,
    amount: U256,
) -> ExecuteRequest {
    match sender {
        Key::Account(sender) => ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            AddressableEntityHash::new(cep18_contract_hash.value()),
            ENTRY_POINT_APPROVE,
            runtime_args! {
                ARG_SPENDER => spender,
                ARG_AMOUNT => amount,
            },
        )
        .build(),
        Key::Hash(contract_package_hash) => ExecuteRequestBuilder::versioned_contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            PackageHash::new(contract_package_hash),
            None,
            ENTRY_POINT_APPROVE_AS_STORED_CONTRACT,
            runtime_args! {
                ARG_TOKEN_CONTRACT => Key::Hash(cep18_contract_hash.value()),
                ARG_SPENDER => spender,
                ARG_AMOUNT => amount,
            },
        )
        .build(),
        Key::AddressableEntity(entity_addr) => match entity_addr {
            EntityAddr::System(_) => panic!("Not a use case"),
            EntityAddr::Account(account_addr) => ExecuteRequestBuilder::contract_call_by_hash(
                AccountHash::new(account_addr),
                AddressableEntityHash::new(cep18_contract_hash.value()),
                ENTRY_POINT_APPROVE,
                runtime_args! {
                    ARG_SPENDER => spender,
                    ARG_AMOUNT => amount,
                },
            )
            .build(),
            EntityAddr::SmartContract(_contract_hash) => panic!("Invalid variant"),
        },
        Key::SmartContract(contract_package_hash) => {
            ExecuteRequestBuilder::versioned_contract_call_by_hash(
                *DEFAULT_ACCOUNT_ADDR,
                PackageHash::new(contract_package_hash),
                None,
                ENTRY_POINT_APPROVE_AS_STORED_CONTRACT,
                runtime_args! {
                    ARG_TOKEN_CONTRACT => Key::Hash(cep18_contract_hash.value()),
                    ARG_SPENDER => spender,
                    ARG_AMOUNT => amount,
                },
            )
            .build()
        }
        _ => panic!("Unknown variant"),
    }
}

pub(crate) fn test_approve_for(
    builder: &mut LmdbWasmTestBuilder,
    test_context: &TestContext,
    sender: Key,
    owner: Key,
    spender: Key,
) {
    let TestContext {
        cep18_contract_hash,
        ..
    } = test_context;
    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    let allowance_amount_1 = U256::from(AMOUNT_ALLOWANCE_1);
    let allowance_amount_2 = U256::from(AMOUNT_ALLOWANCE_2);

    let spender_allowance_before = cep18_check_allowance_of(builder, owner, spender);
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request_1 =
        make_cep18_approve_request(sender, cep18_contract_hash, spender, allowance_amount_1);
    let approve_request_2 =
        make_cep18_approve_request(sender, cep18_contract_hash, spender, allowance_amount_2);

    builder.exec(approve_request_1).expect_success().commit();

    {
        let account_1_allowance_after = cep18_check_allowance_of(builder, owner, spender);
        assert_eq!(account_1_allowance_after, allowance_amount_1);

        let total_supply: U256 = builder.get_value(
            EntityAddr::new_smart_contract(cep18_contract_hash.value()),
            ARG_TOTAL_SUPPLY,
        );
        assert_eq!(total_supply, initial_supply);
    }

    // Approve overwrites existing amount rather than increase it

    builder.exec(approve_request_2).expect_success().commit();

    let account_1_allowance_after = cep18_check_allowance_of(builder, owner, spender);
    assert_eq!(account_1_allowance_after, allowance_amount_2);

    // Swap Key::Account into Hash and other way
    let inverted_spender_key = invert_cep18_address(spender);

    let inverted_spender_allowance = cep18_check_allowance_of(builder, owner, inverted_spender_key);
    assert_eq!(inverted_spender_allowance, U256::zero());

    let total_supply: U256 = builder.get_value(
        EntityAddr::new_smart_contract(cep18_contract_hash.value()),
        ARG_TOTAL_SUPPLY,
    );
    assert_eq!(total_supply, initial_supply);
}
