//! Implementation of the TOML based configuration descriptor reader.
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;
use toml;

use error::{Error, Result};

#[derive(Deserialize)]
pub struct KafkaConfig {
    pub hosts: Vec<String>,
    pub ack_timeout: u64,
    pub topic: String
}

#[derive(Deserialize)]
pub struct Config {
    /// kafka broker related configuration
    pub kafka: KafkaConfig
}

/// Reads the configuration descriptor from the TOML file. It first scans the list of well known
/// locations to find a valid configuration file. If non existing path is found, it fallbacks to
/// resolve the configuration file path from `CUBOSTRATUSC_CONFIG` environment variable.
///
/// If the configuration descriptor can be loaded from any of the available paths and is parsed
/// correctly, this function returns `Result::Ok(config)` where `config` references [Config](struct.
/// Config.html). On error it returns `Result::Err(e)`.
///
pub fn read_config() -> Result<Config> {
    let mut content = String::new();
    let locations = vec!["/etc/cubostratusc.toml", "/var/lib/cubostratusc/cubostratusc.toml"];
    let mut path = locations.iter()
            .find(|loco| Path::new(*loco).exists())
            .map(|l| l.to_string());
    if path == None {
        path = env::var("CUBOSTRATUSC_CONFIG").map(|e| Some(e)).unwrap_or(None);
    }
    if let Some(path) = path {
        File::open(path)
                .unwrap()
                .read_to_string(&mut content)
                .expect("unable to open configuration file");
        toml::from_str(&content).map_err(|e| Error::ConfigParseError(e.to_string()))
    } else {
        Err(Error::UnknownConfigPathError)
    }
}
