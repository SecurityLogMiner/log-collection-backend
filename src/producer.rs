use std::process::{Command, Stdio};
use tokio::time;
use ctrlc;
use std::fs::{File,OpenOptions};
use std::iter::zip;
use std::io::{BufReader, BufRead, Write, Result};
use std::thread;
use std::sync::mpsc::{channel,Sender,Receiver};
use uuid::Uuid;
use crate::config::{Config};
use crate::awssdk;
use aws_sdk_firehose::{Client, types::Record, primitives::Blob};

#[derive(Debug, Clone)]
pub struct DataBuffer {
    name: String,
}

fn 
tail_and_send_log(path: &str, sender: Sender<String>) -> Result<()> {
    let mut tail_process = Command::new("tail")
        .args(["-f","-n0","-q", &path])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = tail_process.stdout.take().expect("Failed to open stdout");

    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                sender.send(line).expect("Failed to send data");
            }
        }
    });
    Ok(())
}

pub fn create_data_buffer() -> Result<DataBuffer> {
    let uuid = Uuid::new_v4();
    let _ = File::create(uuid.to_string())?;
    let mut bf = DataBuffer {
        name: uuid.to_string(),
    };
    Ok(bf)
}

pub fn destroy_data_buffer(name: String) -> Result<()> {
    std::fs::remove_file(name)?;
    Ok(())
}


pub fn insert_into_buffer(mut bf: File, data: &str) -> Result<()> {
    bf.write_all(b"some data should be in amihere.txt")?;
    Ok(())
}

async fn 
handle_log_data(log_channel: Receiver<String>, 
                client: Client, buffer: DataBuffer) {
    let mut buf = OpenOptions::new()
        .write(true)
        .append(true)
        .open(buffer.name.to_string()).expect("issue");
    let mut written = 0;
    let mut testvec = Vec::<Record>::new();
    for log_line in log_channel {
        testvec.push(
            Record::builder()
            .set_data(Some(Blob::new(log_line.clone())))
            .build()
            .expect("error sending the data"),
        );
        written = written + &log_line.chars().count();
        if written > 1000 {
            let res = awssdk::put_record_batch(&client,"PUT-S3-ZG3gK",testvec.clone()).await;
            match res {
                Ok(val) => println!("success: {val:?}"),
                Err(err) => eprintln!("error: {err}"),
            }
            println!("wrote: {written}");
            written = 0;
        }
    }
}

pub async fn 
start_log_stream(config: Config) -> Result<()> {
    let (tx,rx) = channel();
    ctrlc::set_handler(move || {
        println!("handle ctrlc signal");
        tx.send(()).expect("unable to send termination signal");
    }).expect("issue with ctrlc signal handling");

    let mut senders = Vec::new();
    let mut receivers = Vec::new();
    let mut buffers = Vec::<DataBuffer>::new();
    let mut clients = Vec::<Client>::new();

    for input_log_file in config.log_paths.clone().into_iter() {
        // replace this with start_firehose().await. 
        if let Ok(client) = awssdk::start_firehose().await {
            clients.push(client);
        }

        if let Ok(bf) = create_data_buffer() {
            println!("Creating data buffer {}", bf.name);
            buffers.push(bf);
        }

        let (sender, receiver) = channel();
        senders.push(sender);
        receivers.push(receiver);
         
        let sender_clone = senders.last().unwrap().clone();
        thread::spawn(move || {
            tail_and_send_log(&input_log_file, sender_clone)
                .expect("Failed to tail log file");
        });
    }

    let mut count: u8 = 0;
    let iter = zip(receivers.into_iter(), zip(clients,buffers.clone()));
    for (receiver, client_buffer) in iter {
        let (client, buffer) = client_buffer;
        thread::spawn(move || {
            let tokio_handle = tokio::runtime::Runtime::new().unwrap();
                tokio_handle.block_on(async {
                    // the file buffer needs to gt passed into this as well
                    // todo!
                    handle_log_data(receiver, client, buffer).await;
                });
        });
    }

    rx.recv().expect("unable to receive from channel");
    for buf in buffers {
        println!("Deleting {}",buf.name);
        let _ = destroy_data_buffer(buf.name);
    }

    Ok(())
}

#[test]
fn test_the_thing() {
    assert_eq!(1,1);
}
