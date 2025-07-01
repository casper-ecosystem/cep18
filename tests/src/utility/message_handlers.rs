use casper_engine_test_support::LmdbWasmTestBuilder;
use casper_types::{
    contract_messages::{MessageChecksum, MessageTopicSummary, TopicNameHash},
    AddressableEntityHash, Digest, EntityAddr, Key, StoredValue,
};

pub fn message_topic(
    builder: &LmdbWasmTestBuilder,
    contract_hash: &AddressableEntityHash,
    topic_name_hash: TopicNameHash,
) -> MessageTopicSummary {
    let entity_addr = EntityAddr::SmartContract(contract_hash.value());
    let query_result = builder
        .query(None, Key::message_topic(entity_addr, topic_name_hash), &[])
        .expect("should query");

    match query_result {
        StoredValue::MessageTopic(summary) => summary,
        _ => {
            panic!("Stored value is not a message topic summary: {query_result:?}",);
        }
    }
}

pub fn message_summary(
    builder: &LmdbWasmTestBuilder,
    contract_hash: &AddressableEntityHash,
    topic_name_hash: &TopicNameHash,
    message_index: u32,
    state_hash: Option<Digest>,
) -> Result<MessageChecksum, String> {
    let entity_addr = EntityAddr::SmartContract(contract_hash.value());
    let query_result = builder.query(
        state_hash,
        Key::message(entity_addr, *topic_name_hash, message_index),
        &[],
    )?;

    match query_result {
        StoredValue::Message(summary) => Ok(summary),
        _ => panic!("Stored value is not a message summary: {query_result:?}"),
    }
}
