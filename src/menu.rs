use crate::config::manage_collection_configurations;
use crate::producer::{start_log_service, stop_log_service, list_available_logs, view_running_logs};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::watch;



pub fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    input.trim().to_string()
}

// fn write_to_file(message: &str, file_path: &str) -> io::Result<()> {
//     let current_time = Utc::now();
//     let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
//     let message_with_time = format!("[{}] {}", formatted_time, message);
//     let mut file = OpenOptions::new().create(true).append(true).open(file_path)?;
//     writeln!(file, "{}", message_with_time)?;
//     Ok(())
// }

pub fn display_menu() {
    println!("Menu:");
    println!("1. Start Log Collection service");
    println!("2. Stop Log Collection service");
    println!("3. View Running Log Collection services");
    println!("4. Manage Log Collection configurations");
    println!("5. Exit");
}

pub async fn handle_menu_choice(choice: &str, log_services: Arc<Mutex<HashMap<String, watch::Sender<()>>>>) {
    match choice {
        "1" => {

            // Look for the avilable logs and iterate them if there exists any
            // clone the log_sercies to pass it to the start_log_service
            // use tokio::spawn to start the log service
            // use tokio::join to wait for the log service to start
            let available_logs = list_available_logs();
            if available_logs.is_empty() {
                println!("No available logs to start.");
                return;
            }

            // Look for the available logs that have not been started
            // clone the log_services to pass it to the start_log_service
            let services = log_services.lock().unwrap();
            let filtered_logs: Vec<_> = available_logs
                .into_iter()
                .filter(|log| !services.contains_key(log))
                .collect();
            if filtered_logs.is_empty() {
                println!("No available logs to start. All logs are already running.");
                return;
            }

            println!("Available log services to start:");
            for (index, log) in filtered_logs.iter().enumerate() {
                println!("{}. {}", index + 1, log);
            }

            let log_choice = read_input("Enter the number of the log service to start: ");
            if let Ok(index) = log_choice.parse::<usize>() {
                if index > 0 && index <= filtered_logs.len() {
                    let service_name = filtered_logs[index - 1].clone();
                    let log_services = log_services.clone();
                    tokio::spawn(async move {
                        start_log_service(service_name, log_services).await;
                    });
                } else {
                    println!("Invalid choice.");
                }
            } else {
                println!("Invalid input.");
            }
        }
        // Lock the log_services and stop the log service
        // Clone the log_services to pass it to the stop_log_service
        // Grab the name of the log service based on the index
        // Use tokio::spawn to stop the log service
        "2" => {

            println!("Available running log services to stop:");
            view_running_logs(log_services.clone());
            let services = log_services.lock().unwrap();
            let service_names: Vec<_> = services.keys().cloned().collect();
            drop(services); // Release the lock before awaiting
            let log_choice = read_input("Enter the number of the log service to stop: ");
            if let Ok(index) = log_choice.parse::<usize>() {
                if index > 0 && index <= service_names.len() {
                    let service_name = service_names[index - 1].clone();
                    let log_services = log_services.clone();
                    stop_log_service(service_name, log_services).await;
                } else {
                    println!("Invalid choice.");
                }
            } else {
                println!("Invalid input.");
            }
        }
        "3" => view_running_logs(log_services),
        "4" => manage_collection_configurations(),
        "5" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => println!("Invalid choice"),
    }
}

