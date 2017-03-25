//! Syscall's stream aggregators used to ingest the flow of syscall events from the collector
//! to messaging systems.

use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;
use std::marker::Sized;
use kafka;

pub trait Aggregator<E, C> {
    fn boot(config: C) -> Result<Self, E> where Self: Sized;

    fn do_aggregate(&mut self, body: String);
}

pub struct KafkaAggregator {
    producer: Producer,
    topic: &'static str
}

pub struct KafkaConfig {
    pub hosts: Vec<String>,
    pub ack_timeout: Duration,
    pub topic: &'static str
}

/// Implementation of the syscall's aggregator which emits the stream of syscall events
/// to Kafka brokers.
impl Aggregator<kafka::Error, KafkaConfig> for KafkaAggregator {

    fn boot(config: KafkaConfig) -> Result<KafkaAggregator, kafka::Error> {
        let producer = match Producer::from_hosts(config.hosts)
                .with_ack_timeout(config.ack_timeout)
                .create() {
                Ok(p) => p,
                Err(e) => return Err(e)
        };
        Ok(KafkaAggregator {
            topic: config.topic,
            producer: producer
        })
    }

    fn do_aggregate(&mut self, body: String) {
        self.producer.send(&Record::from_value(self.topic, body.as_bytes())).unwrap();
    }
}