use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Result};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use tokio::sync::watch;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::config::read_config;
use crate::dynamosdk;
use crate::traits::DataHandler;

// Tail the log file and send the data to the channel until the shutdown signal is received
// spawn a new thread to tail the log file
// spawn a new thread to send the data to the channel
// use tokio::sync::watch to receive the shutdown signal
fn tail_and_send_log(path: &str, sender: Sender<(String, String)>, shutdown_rx: watch::Receiver<()>) -> Result<()> {
    let mut tail_process = Command::new("tail")
        .args(["-f", "-n0", "-q", path])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = tail_process.stdout.take().expect("Failed to open stdout");

    let _receiver = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                if shutdown_rx.has_changed().unwrap_or(false) {
                    break;
                }
                let tup = ("DateGoesHere".to_string(), line);
                sender.send(tup).expect("Failed to send data");
            }
        }
    });

    Ok(())
}
// List the available log services from the config
// return a vector of strings representing the log services
pub fn list_available_logs() -> Vec<String> {
    let config_data = read_config();
    if let Some(config) = config_data {
        config.dynamodb.package.into_iter().map(|p| p.source).collect()
    } else {
        vec![]
    }
}

// Start the log service and tail the log file
// spawn a new thread to tail the log file
// spawn a new thread to send the data to the channel
// Create a shutdown channel and use tokio::sync::watch to receive the shutdown signal
// Insert the service name and the shutdown channel into the log services

pub async fn start_log_service(service_name: String, log_services: Arc<Mutex<HashMap<String, watch::Sender<()>>>>) {
    let config_data = read_config();
    match config_data {
        Some(config) => {
            let (shutdown_tx, shutdown_rx) = watch::channel(());

            {
                let mut services = log_services.lock().unwrap();
                services.insert(service_name.clone(), shutdown_tx);
            }

            for package in config.dynamodb.package {
                if package.source == service_name {
                    let (sender, receiver) = channel::<(String, String)>();
                    let sender_clone = sender.clone();
                    let source = package.source.clone();

                    tokio::task::spawn_blocking(move || {
                        tail_and_send_log(&source, sender_clone, shutdown_rx.clone()).expect("Failed to tail log file");
                    });

                    let client = dynamosdk::create_client(package.table).await.unwrap();
                    tokio::spawn(async move {
                        client.handle_log_data(receiver).await;
                    });

                    break;
                }
            }
        }
        None => panic!("Error reading configuration."),
    }
}

// Stop the log service and remove the service name from the log services
// use tokio::sync::watch to send the shutdown signal
// send the shutdown signal if the service name exists in the log services
pub async fn stop_log_service(service_name: String, log_services: Arc<Mutex<HashMap<String, watch::Sender<()>>>>) {
    let mut services = log_services.lock().unwrap();
    
    if let Some(shutdown_tx) = services.remove(&service_name) {
        shutdown_tx.send(()).unwrap();
        println!("Stopped log service: {}", service_name);
    } else {
        println!("Log service not found: {}", service_name);
    }
}

// View the running log services

pub fn view_running_logs(log_services: Arc<Mutex<HashMap<String, watch::Sender<()>>>>) {
    let services = log_services.lock().unwrap();
    if services.is_empty() {
        println!("No running log services.");
    } else {
        println!("Currently running log services:");
        for (service_name, _) in services.iter() {
            println!("- {}", service_name);
        }
    }
}
