use crate::producer;
use crate::config::read_config;
use std::io::{self, Write};
use std::fs::OpenOptions;
use chrono::{Utc, Datelike, Timelike};
use tokio::sync::watch;
use crate::iam;

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

pub fn display_menu() {
    println!("Menu:");
    println!("1. Start Log Collection service");
    println!("2. Stop Log Collection service");
    println!("3. View Log Collection service status");
    println!("4. Manage Log Collection configurations");
    println!("5. Backup Log Collection data");
    println!("6. Restore Log Collection data");
    println!("7. Exit");
}

pub async fn start_collection_service(mut shutdown_rx: watch::Receiver<()>) {
    let config_data = read_config();
    match config_data {
        Some(config) => {
            println!("{:?}", config);

            if let Err(e) = producer::start_log_stream(config.dynamodb, shutdown_rx).await {
                let str_error = format!("Log stream error: {}", e);
                write_to_file(&str_error, "collection.log").expect("Failed to write to file");
            }
            write_to_file("Starting Log Collection service...", "collection.log").expect("Failed to write to file");
            println!("Starting Log Collection service...");
        }
        None => panic!("Error reading configuration."),
    }
}

pub fn stop_collection_service(shutdown_tx: &watch::Sender<()>) {
    println!("Stopping Log Collection service...");
    shutdown_tx.send(()).unwrap();
    write_to_file("Stopping Log Collection service...", "collection.log").expect("Failed to write to file");
}

pub async fn handle_menu_choice(choice: &str, shutdown_tx: &watch::Sender<()>, shutdown_rx: watch::Receiver<()>) {
    match choice {
        "1" => {
            let shutdown_rx = shutdown_rx.clone();
            tokio::spawn(async move {
                start_collection_service(shutdown_rx).await;
            });
        }
        "2" => {
            println!("Stopping Log Collection service...");
            stop_collection_service(shutdown_tx); // Send shutdown signal
            write_to_file("Stopping Log Collection service...", "collection.log").expect("Failed to write to file");
        }
        "3" => view_collection_service_status(),
        "4" => manage_collection_configurations(),
        "5" => backup_collection_data(),
        "6" => restore_collection_data(),
        "7" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => println!("Invalid choice"),
    }
}

fn view_collection_service_status() {
    println!("Viewing Log Collection service status...");
    write_to_file("Viewing Log Collection service status...", "collection.log").expect("Failed to write to file");
}

fn manage_collection_configurations() {
    println!("Managing Log Collection configurations...");
    write_to_file("Managing Log Collection configurations...", "collection.log").expect("Failed to write to file");
}

fn backup_collection_data() {
    println!("Backing up Log Collection data...");
    write_to_file("Backing up Log Collection data...", "collection.log").expect("Failed to write to file");
}

fn restore_collection_data() {
    println!("Restoring Log Collection data...");
    write_to_file("Restoring Log Collection data...", "collection.log").expect("Failed to write to file");
}

pub async fn admin_cli() {
    println!("Running AWS Administrator CLI...");
    if let Err(e) = iam::run_admin_cli().await {
        let str_error = format!("AWS Administrator CLI error: {}", e);
        write_to_file(&str_error, "collection.log").expect("Failed to write to file");
    }
    write_to_file("Running AWS Administrator CLI...", "collection.log").expect("Failed to write to file");
}
