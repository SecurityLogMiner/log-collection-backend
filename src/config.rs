use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

const CONFIG_PATH: &str = "config.toml";
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Package {
    pub source: String,
    pub table: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DynamoDBConfig {
    pub package: Vec<Package>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub dynamodb: DynamoDBConfig,
}

// Read the config toml file
pub fn read_config() -> Option<Config> {
    let mut file = File::open(CONFIG_PATH).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let config: Config = toml::from_str(&data).unwrap();
    Some(config)
}

// Write the config toml file
pub fn write_config<P: AsRef<Path>>(path: P, config: &Config) -> std::io::Result<()> {
    let config_data = toml::to_string_pretty(config).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    fs::write(path, config_data)
}

// Add a new log service to the config
pub fn add_log_config(config: &mut Config, source: String, table: String) {
    let new_package = Package { source, table };
    config.dynamodb.package.push(new_package);
}

// Remove a log service from the config
pub fn remove_log_config(config: &mut Config, source: &str) -> bool {
    if let Some(pos) = config.dynamodb.package.iter().position(|p| p.source == source) {
        config.dynamodb.package.remove(pos);
        true
    } else {
        false
    }
}

// Update a log service in the config
pub fn update_log_config(config: &mut Config, source: &str, new_table: String) -> bool {
    if let Some(package) = config.dynamodb.package.iter_mut().find(|p| p.source == source) {
        package.table = new_table;
        true
    } else {
        false
    }
}

// Manage the log service configurations
pub fn manage_collection_configurations() {
    const CONFIG_PATH: &str = "config.toml";

    fn read_input(prompt: &str) -> String {
        use std::io::{self, Write};
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        input.trim().to_string()
    }

    println!("Configuration Management:");
    println!("1. Add Log Configuration");
    println!("2. Remove Log Configuration");
    println!("3. Update Log Configuration");

    let choice = read_input("Enter your choice: ");
    match choice.as_str() {
        "1" => { 
            let source = read_input("Enter log source: ");
            let table = read_input("Enter DynamoDB table name: ");
            let mut config = read_config().expect("Failed to read config");
            
            add_log_config(&mut config, source, table);
            write_config(CONFIG_PATH, &config).expect("Failed to write config");
            println!("Log configuration added.");
        }
        "2" => {
            let source = read_input("Enter log source to remove: ");
            let mut config = read_config().expect("Failed to read config");
            if remove_log_config(&mut config, &source) {
                write_config(CONFIG_PATH, &config).expect("Failed to write config");
                println!("Log configuration removed.");
            } else {
                println!("Log source not found.");
            }
        }
        "3" => {
            let source = read_input("Enter log source to update: ");
            let new_table = read_input("Enter new DynamoDB table name: ");
            let mut config = read_config().expect("Failed to read config");
            if update_log_config(&mut config, &source, new_table) {
                write_config(CONFIG_PATH, &config).expect("Failed to write config");
                println!("Log configuration updated.");
            } else {
                println!("Log source not found.");
            }
        }
        _ => println!("Invalid choice."),
    }
}
