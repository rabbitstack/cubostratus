#[macro_use]
extern crate nix;
extern crate libc;
extern crate num_cpus;
extern crate chrono;
extern crate kafka;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::default::Default;
use std::process;

mod collector;
mod syscall;
mod aggregator;
mod error;
mod value;

use collector::Collector;
use collector::RingBufferCollector;
use syscall::syscall_table::SyscallTable;
use aggregator::{Aggregator, KafkaAggregator, KafkaConfig};

use std::time::Duration;

fn main() {

    let mut collector = RingBufferCollector::new();
    match collector.start() {
        Ok(num_devs) => {

        },
        Err(e) =>  {
            println!("error {}", e);
            process::exit(0);
        }
    }

    let syscall_table = SyscallTable::default();
    let mut aggregator = KafkaAggregator::boot(KafkaConfig {
        topic: "cubostratus",
        hosts: vec!["localhost:9092".to_string()],
        ack_timeout: Duration::from_secs(1)
    }).unwrap();

    loop {
        match collector.next() {
            Some(syscall) => {
                let json = serde_json::to_string(&syscall).unwrap();
                aggregator.do_aggregate(json);
            },
            None => {}
        }
    }
}
