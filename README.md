<p align="center">
  <img width="500" height="98" src="https://github.com/rabbitstack/cubostratusc/blob/master/cubostratus.png" />
</p>

**cubostratusc** (**c** stands for collector) is part of **cubostratus** - distributed instrumentation platform with emphasis on containers and microservice envrionments. It is still under heavy development.

cubostratusc acquires the syscall flow from the rock solid [sysdig's](https://github.com/draios/sysdig) driver and emits it to Kafka brokers for later ingestion, storage and analysis.

# Usage

1. Build the sysdig kernel module or [install](http://www.sysdig.org/install/) sysdig
2. Install Rust
```bash
curl -f -L https://static.rust-lang.org/rustup.sh -O
sh rustup.sh
```
3. Clone this repository and build `cubostratusc` 
```bash
git clone https://github.com/rabbitstack/cubostratusc.git
cd cubostratusc
cargo build
````
4. Run it (you'll need a Kafka broker running)
```bash
sudo ./target/debug/cubostratusc
```
