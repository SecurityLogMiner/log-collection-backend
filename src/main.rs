#![allow(unused_imports)]
#![allow(dead_code)]
mod traits;
mod config;
mod producer;
mod dynamosdk;
mod menu;
mod util;
mod iam;

use aws_config::imds::Client;
use producer::start_log_stream;
use config::read_config;
use std::{env, process};
use util::{print_help};
use menu::{show_menu};
/*
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    //let _ = show_menu();

    if args.len() <= 2 {
        let config_data = read_config();

        match config_data {
            Some(config) => {
                if args.len() == 1 {

                //Create a setup functoin 
                // User gives IAM credentials; as long as they have correct policies; based on the policies set up on whatever they have available.
                // Attach policies to IAM user based on the set up function
                    todo!();
            }
                if args.len() == 2 {
                    if args[1] == "--help" || args[1] == "-h" {
                        util::print_help().await;
                    }

                    let destination = args[1].as_str();
                    println!("Destination: {}", destination);
                    match destination {
                        "dynamodb" => {
                            //dynamosdk::send_dynamodb(config).await;
                            let _ = start_log_stream(config.dynamodb).await;
                        }
                        "iam" => {
                            util::initialize_iam(config).await;
                        }
                        "run-admin" => {
                            // util::initialize_iam(config).await;
                            util::run_admin_cli().await;
                        }
                        _ => {
                            util::print_help().await;
                        }
                    }
                }
            }
            None => panic!("Error reading configuration. Fix it."),
        }
    } 

    println!("testing systemd");

    Ok(())
}
*/

use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use chrono::{Utc, Datelike, Timelike};

#[tokio::main]
async fn main() {

    loop {
        println!("Menu:");
        println!("1. Start Log Collection service");
        println!("2. Stop Log Collection service");
        println!("3. View Log Collection service status");
        println!("4. Manage Log Collection configurations");
        println!("5. Backup Log Collection data");
        println!("6. Restore Log Collection data");
        println!("7. Exit");

        let choice = read_input("Enter your choice: ");
        match choice.trim() {
            "1" => start_collection_service().await,
            "2" => stop_collection_service(),
            "3" => view_collection_service_status(),
            "4" => manage_collection_configurations(),
            "5" => backup_collection_data(),
            "6" => restore_collection_data(),
            "7" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn read_input(prompt: &str) -> String {
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

async fn start_collection_service() {
    /*
    let config_data = read_config();
    match config_data {
        Some(config) => {
            println!("{config:?}");

            if let Err(e) = start_log_stream(config.dynamodb).await {
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
    */
    write_to_file("Starting Log Collection service...", 
                  "collection.log").expect("Failed to write to file");
    println!("Starting Log Collection service...");
}

fn stop_collection_service() {
    println!("Stopping Log Collection service...");
    write_to_file("Stopping Log Collection service...", "collection.log").expect("Failed to write to file");
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
