
use crate::producer;
use crate::config::read_config;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::fs;
use chrono::{Utc, Datelike, Timelike};

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    input
}

fn write_to_file(message: &str, file_path: &str) -> io::Result<()> {
    let current_time = Utc::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
    let message_with_time = format!("[{}] {}", formatted_time, message);
    let mut file = OpenOptions::new().create(true).append(true).open(file_path)?;
    writeln!(file, "{}", message_with_time)?;
    Ok(())
}

// Start a process in the background that handles all threads. 
// The threads represent incoming log data from the log file.
// Each thread has a transmitter and receiver, aka a channel that needs
// to run asynchronously, in the background.
pub async fn start_collection_service() {
    
    let config_data = read_config();
    match config_data {
        Some(config) => {
            println!("{config:?}");

            if let Err(e) = producer::start_log_stream(config.dynamodb).await {
                let str_error = format!("Log stream error: {}", e);
                write_to_file(&str_error, 
                    "collection.log").expect("Failed to write to file");
            }
            write_to_file("Starting Log Collection service...", 
                          "collection.log").expect("Failed to write to file");
            println!("Starting Log Collection service...");

        }
        None => panic!("Error reading configuration."),
    }
    write_to_file("Starting Log Collection service...", 
                  "collection.log").expect("Failed to write to file");
    println!("Starting Log Collection service...");
}

pub fn stop_collection_service() {
    println!("Stopping Log Collection service...");
    write_to_file("Stopping Log Collection service...", "collection.log").expect("Failed to write to file");
}

pub fn view_collection_service_status() {
    println!("Viewing Log Collection service status...");
    write_to_file("Viewing Log Collection service status...", "collection.log").expect("Failed to write to file");
}

pub fn manage_collection_configurations() {
    println!("Managing Log Collection configurations...");
    write_to_file("Managing Log Collection configurations...", "collection.log").expect("Failed to write to file");
}

pub fn backup_collection_data() {
    println!("Backing up Log Collection data...");
    write_to_file("Backing up Log Collection data...", "collection.log").expect("Failed to write to file");
}

pub fn restore_collection_data() {
    println!("Restoring Log Collection data...");
    write_to_file("Restoring Log Collection data...", "collection.log").expect("Failed to write to file");
}
