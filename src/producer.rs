use std::process::{Command, Stdio};
use tokio::time;
use ctrlc;
use async_trait::async_trait;
use std::fs::{File,OpenOptions};
use std::iter::zip;
use std::io::{BufReader, BufRead, Write, Result};
use std::thread;
use std::sync::{Arc, mpsc::{channel,Sender,Receiver}};
use uuid::Uuid;
use crate::config::{Config, DynamoDBConfig, Package};
use crate::dynamosdk;
use crate::traits::DataHandler;
use aws_sdk_dynamodb::Client as DynamodbClient;
use aws_sdk_dynamodb::types::AttributeValue;


// Tail the log file and send the data to the table
fn 
tail_and_send_log(path: &str, 
                  sender: Sender<(String,String)>) -> Result<()> {
    let mut tail_process = Command::new("tail")
        .args(["-f","-n0","-q", &path])
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

// Start the log stream
// and send the data to the table specified in the config
// Sender and Receiver are used to send and receive data.
// Threads are used to tail the log file and send the data.
// The table is created if it doesn't exist.
pub async fn 
start_log_stream(config: DynamoDBConfig) -> Result<()> {
    println!("{config:?}");

    let (tx,rx) = channel();
    ctrlc::set_handler(move || {
        println!("handle ctrlc signal");
        tx.send(()).expect("unable to send termination signal");
    }).expect("issue with ctrlc signal handling");
    
    let mut senders = Vec::new();
    let mut receivers = Vec::new();
    let mut clients = Vec::<_>::new();
    // let mut log_count = 0;

    for package in config.package {
        // log_count += 1;
        if let Ok(client) = dynamosdk::create_client(package.table).await {
            clients.push(client);
        }
 
        // channel tuple (time, data)
        let (sender, receiver) = channel::<(String, String)>();
        senders.push(sender);
        receivers.push(receiver);
         
        let sender_clone = senders.last().unwrap().clone();
        
        thread::spawn(move || {
            tail_and_send_log(&package.source, sender_clone)
                .expect("Failed to tail log file");
        });
    }

    let iter = zip(receivers.into_iter(), clients);
    for (receiver, wrapper) in iter {
        thread::spawn(move || {
            let tokio_handle = tokio::runtime::Runtime::new().unwrap();
            tokio_handle.block_on(async {
                wrapper.handle_log_data(receiver).await;
            });
        });
    }

    rx.recv().expect("unable to receive from channel");

    Ok(())
}


#[test]
fn test_the_thing() {
    assert_eq!(1,1);
}
