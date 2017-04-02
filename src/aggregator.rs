//! Syscall's stream aggregators used to ingest the flow of syscall events from the collector
//! to messaging systems.
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka;
use config::KafkaConfig;

pub trait Aggregator<T> {

    fn do_aggregate(&mut self, body: T);
}

pub struct KafkaAggregator {
    /// an instance of the Kafka producer
    producer: Option<Producer>,
    /// kafka configuration
    config: KafkaConfig
}

/// Implementation of the syscall's aggregator which emits the stream of syscall events
/// to Kafka brokers.
impl Aggregator<String> for KafkaAggregator {

    fn do_aggregate(&mut self, body: String) {
        match self.producer {
            Some(ref mut p) => {
                p.send(&Record::from_value(&self.config.topic, body.as_bytes())).unwrap();
            }
            None => {}
        }
    }
}

impl KafkaAggregator {

    pub fn new(config: KafkaConfig) -> KafkaAggregator {
        KafkaAggregator {
            producer: None,
            config: config
        }
    }

    pub fn start(&mut self) -> Result<(), kafka::Error> {
        self.producer = match Producer::from_hosts(self.config.hosts.clone())
                .with_ack_timeout(Duration::from_secs(self.config.ack_timeout))
                .create() {
                    Ok(p) => Some(p),
                    Err(e) => return Err(e)
        };
        Ok(())
    }
}