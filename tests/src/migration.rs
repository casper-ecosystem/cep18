use casper_engine_test_support::{
    ExecuteRequestBuilder, LmdbWasmTestBuilder, UpgradeRequestBuilder, DEFAULT_ACCOUNT_ADDR,
};
use casper_fixtures::LmdbFixtureState;
use casper_types::{
    runtime_args, AddressableEntityHash, EntityAddr, EraId, Key, ProtocolVersion, RuntimeArgs, U256,
};
use cep18::{
    constants::{ARG_AMOUNT, ARG_EVENTS, ARG_EVENTS_MODE, ARG_NAME, ARG_OWNER, ENTRY_POINT_MINT},
    modalities::EventsMode,
};

use crate::utility::{
    constants::{
        AMOUNT_1, CEP18_CONTRACT_WASM, CEP18_TEST_CONTRACT_WASM, CEP18_TEST_TOKEN_CONTRACT_NAME,
        CEP18_TEST_TOKEN_CONTRACT_VERSION, TOKEN_NAME,
    },
    installer_request_builders::{cep18_check_balance_of, get_test_account},
    message_handlers::{message_summary, message_topic},
    support::query_stored_value,
};

pub fn upgrade_v1_5_6_fixture_to_v2_0_0_ee(
    builder: &mut LmdbWasmTestBuilder,
    lmdb_fixture_state: &LmdbFixtureState,
) {
    // state hash in builder and lmdb storage should be the same
    assert_eq!(
        builder.get_post_state_hash(),
        lmdb_fixture_state.post_state_hash
    );

    // we upgrade the execution engines protocol from 1.x to 2.x
    let mut upgrade_config = UpgradeRequestBuilder::new()
        .with_current_protocol_version(lmdb_fixture_state.genesis_protocol_version())
        .with_new_protocol_version(ProtocolVersion::V2_0_0)
        // TODO GR
        // .with_migrate_legacy_accounts(true)
        // .with_migrate_legacy_contracts(true)
        .with_activation_point(EraId::new(1))
        .build();

    builder
        .upgrade(&mut upgrade_config)
        .expect_upgrade_success()
        .commit();

    // the state hash should now be different
    assert_ne!(
        builder.get_post_state_hash(),
        lmdb_fixture_state.post_state_hash
    );
}

// the difference between the two is that in v1_binary the contract hash is fetched at [u8;32], while in v2_binary it is an AddressaleEntityHash
pub fn get_contract_hash_v1_binary(builder: &LmdbWasmTestBuilder) -> AddressableEntityHash {
    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .unwrap();
    let account_named_keys = account.named_keys();

    let cep18_token = account_named_keys
        .get(CEP18_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_hash_addr())
        .map(AddressableEntityHash::new)
        .expect("should have contract hash");

    cep18_token
}

pub fn get_contract_hash_v2_binary(builder: &LmdbWasmTestBuilder) -> AddressableEntityHash {
    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .unwrap();
    let account_named_keys = account.named_keys();

    let cep18_token = account_named_keys
        .get(CEP18_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_entity_hash())
        .expect("should have contract hash");

    cep18_token
}

#[test]
fn should_be_able_to_call_1x_contract_in_2x_execution_engine() {
    // load fixture that was created in a previous EE version
    let (mut builder, lmdb_fixture_state, _temp_dir) =
        casper_fixtures::builder_from_global_state_fixture("cep18-1.5.6-minted");

    // upgrade the execution engine to the new protocol version
    upgrade_v1_5_6_fixture_to_v2_0_0_ee(&mut builder, &lmdb_fixture_state);

    let cep18_token = get_contract_hash_v1_binary(&builder);

    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();
}

#[test]
fn should_migrate_1_5_6_to_2_0_0() {
    // load fixture
    let (mut builder, lmdb_fixture_state, _temp_dir) =
        casper_fixtures::builder_from_global_state_fixture("cep18-1.5.6-minted");

    // upgrade engine
    upgrade_v1_5_6_fixture_to_v2_0_0_ee(&mut builder, &lmdb_fixture_state);

    let version_0_major: u32 = 1;
    let version_0_minor: u32 = query_stored_value(
        &builder,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
        CEP18_TEST_TOKEN_CONTRACT_VERSION,
    );

    // upgrade the contract itself using a binary built for the new engine
    let upgrade_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_CONTRACT_WASM,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_EVENTS_MODE => EventsMode::Native as u8,
        },
    )
    .build();

    builder.exec(upgrade_request).expect_success().commit();

    let version_1_string: String = query_stored_value(
        &builder,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
        CEP18_TEST_TOKEN_CONTRACT_VERSION,
    );

    // Split into major and minor parts
    let parts: Vec<&str> = version_1_string.split('.').collect();

    // Parse the major and minor components
    let version_1_major: u32 = parts
        .first()
        .expect("Failed to get the major version")
        .parse()
        .expect("Failed to parse the major version as u32");

    let version_1_minor: u32 = parts
        .get(1)
        .unwrap_or(&"0") // Default to "0" if no minor version exists
        .parse()
        .expect("Failed to parse the minor version as u32");

    assert!(version_0_major < version_1_major);
    assert!(version_0_minor == version_1_minor);

    let cep18_contract_hash = get_contract_hash_v2_binary(&builder);
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    // mint some new tokens in cep-18
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_contract_hash,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let test_contract = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_TEST_CONTRACT_WASM,
        RuntimeArgs::default(),
    )
    .build();

    builder.exec(test_contract).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_contract_hash, account_user_1_key),
        U256::from(AMOUNT_1),
    );
}

#[test]
fn should_have_native_events() {
    let (mut builder, lmdb_fixture_state, _temp_dir) =
        casper_fixtures::builder_from_global_state_fixture("cep18-1.5.6-minted");

    upgrade_v1_5_6_fixture_to_v2_0_0_ee(&mut builder, &lmdb_fixture_state);

    let upgrade_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP18_CONTRACT_WASM,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_EVENTS_MODE => EventsMode::NativeBytes as u8,
        },
    )
    .build();

    builder.exec(upgrade_request).expect_success().commit();

    let cep18_token = get_contract_hash_v2_binary(&builder);

    // events check
    let entity_addr = EntityAddr::SmartContract(cep18_token.value());
    let binding = builder.message_topics(None, entity_addr).unwrap();
    let (topic_name, message_topic_hash) = binding
        .iter()
        .last()
        .expect("should have at least one topic");

    assert_eq!(topic_name, &ARG_EVENTS.to_string());

    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        ENTRY_POINT_MINT,
        runtime_args! {ARG_OWNER => account_user_1_key, ARG_AMOUNT => U256::from(AMOUNT_1)},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    assert_eq!(
        message_topic(&builder, &cep18_token, *message_topic_hash).message_count(),
        2
    );

    message_summary(&builder, &cep18_token, message_topic_hash, 0, None).unwrap();
}
