use std::io::Read;
use std::fs::File;
use toml;
use serde_derive::{Deserialize,Serialize};

#[derive(Debug, Deserialize)]
pub struct Package {
    pub source: String,
    pub table: String,
}

#[derive(Debug, Deserialize)]
pub struct DynamoDBConfig {
    pub package: Vec<Package>
}

// Define a configuration struct above and throw it in the mix
#[derive(Debug, Deserialize)]
pub struct Config {
    pub dynamodb: DynamoDBConfig,
}

pub fn
read_config() -> Option<Config> {
    let mut file = File::open("config.toml").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let config: Config = toml::from_str(&data).unwrap();
    Some(config)
}
