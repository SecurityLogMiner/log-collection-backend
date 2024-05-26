use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Result};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use tokio::sync::watch;
use crate::config::DynamoDBConfig;
use crate::dynamosdk;
use crate::traits::DataHandler;

fn tail_and_send_log(path: &str, sender: Sender<(String, String)>) -> Result<()> {
    let mut tail_process = Command::new("tail")
        .args(["-f", "-n0", "-q", path])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = tail_process.stdout.take().expect("Failed to open stdout");

    let _receiver = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                let tup = ("DateGoesHere".to_string(), line);
                sender.send(tup).expect("Failed to send data");
            }
        }
    });

    Ok(())
}

pub async fn start_log_stream(config: DynamoDBConfig, mut shutdown: watch::Receiver<()>) -> Result<()> {
    println!("{:?}", config);

    let mut clients = Vec::new();

    for package in config.package {
        if let Ok(client) = dynamosdk::create_client(package.table).await {
            clients.push(client);
        }

        let (sender, receiver) = channel::<(String, String)>();
        let sender_clone = sender.clone();

        let source = package.source.clone();

        tokio::task::spawn_blocking(move || {
            tail_and_send_log(&source, sender_clone).expect("Failed to tail log file");
        });

        let wrapper = clients.last().unwrap().clone();
        tokio::spawn(async move {
            wrapper.handle_log_data(receiver).await;
        });
    }

    // Wait for shutdown signal
    shutdown.changed().await.unwrap();
    println!("Shutdown signal received. Stopping log collection.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_thing() {
        assert_eq!(1, 1);
    }
}
