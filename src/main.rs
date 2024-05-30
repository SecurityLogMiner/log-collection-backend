use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod config;
mod producer;
mod dynamosdk;
mod traits;
mod menu;

use menu::{display_menu, handle_menu_choice, read_input};

#[tokio::main]
async fn main() {
    // Initialize the logger
    // env_logger::init();
    println!("Starting the application");

    // Create a shared state to manage log services
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html
    // https://itsallaboutthebit.com/arc-mutex/
    // Using the hashmap to manage the log services
    // Mutex is used to protect the hashmap from concurrent access
    // watch is used to send a signal to the consumer when the log service is started or stopped
    let log_services = Arc::new(Mutex::new(HashMap::new()));

    loop {
        display_menu();
        let choice = read_input("Enter your choice: ");
        // Clone the Arc pointer to pass it to the async task then prompt for the user choice
        let log_services = log_services.clone();
        handle_menu_choice(choice.trim(), log_services).await;

        if choice.trim() == "5" {
            break;
        }
    }
}
