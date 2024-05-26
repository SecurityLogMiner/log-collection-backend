use tokio::sync::watch;
use std::io;

mod config;
mod producer;
mod dynamosdk;
mod traits;
mod util;
mod iam;
mod menu;

use menu::{display_menu, handle_menu_choice, read_input};

#[tokio::main]
async fn main() {
    // Initialize the logger
    println!("Starting the application");

    // Create a channel for shutdown signal
    let (shutdown_tx, shutdown_rx) = watch::channel(());

    loop {
        display_menu();
        let choice = read_input("Enter your choice: ");
        // Clone the receiver each time to pass a new instance
        let shutdown_rx = shutdown_rx.clone();
        handle_menu_choice(choice.trim(), &shutdown_tx, shutdown_rx).await;

        if choice.trim() == "7" {
            break;
        }
    }
}
