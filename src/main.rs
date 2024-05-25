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
use menu::{start_collection_service, read_input, stop_collection_service, view_collection_service_status, manage_collection_configurations, backup_collection_data, restore_collection_data};

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

