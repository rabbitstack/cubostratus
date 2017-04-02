#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate cubostratusc;

use std::default::Default;
use std::process;

use cubostratusc::collector::Collector;
use cubostratusc::collector::RingBufferCollector;
use cubostratusc::syscall::syscall_table::SyscallTable;
use cubostratusc::aggregator::{Aggregator, KafkaAggregator};
use cubostratusc::config;

fn main() {

    let syscall_table = SyscallTable::default();

    let config = match config::read_config() {
        Ok(config) => config,
        Err(e) => {
            exit_process(e.to_string());
        }
    };

    let mut aggregator = KafkaAggregator::new(config.kafka);
    match aggregator.start() {
        Ok(()) => {},
        Err(e) => {
            exit_process(e.to_string());
        }
    }

    let mut collector = RingBufferCollector::new();
    match collector.start() {
        Ok(num_devs) => {
            loop {
                match collector.next() {
                    Some(syscall_info) => {
                        let json = serde_json::to_string(&syscall_info).unwrap();
                        aggregator.do_aggregate(json);
                    },
                    None => {}
                }
            }
        },
        Err(e) =>  {
            exit_process(e.to_string());
        }
    }
}

fn exit_process(e: String) -> ! {
    println!("{}", e);
    process::exit(0)
}