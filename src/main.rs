// Read from a file and detect when new data is appended to that file
<<<<<<< HEAD
use std::process::Command;

fn main() {
    run_tail_f();
}

/*
 * The 'Producer' as defined in the system design file.
 *  - https://github.com/SecurityLogMiner/log-collection-backend/tree/features
 * This function should ideally take a Path parameter. The goal here is to
 * read new data that has been appended to the file and send it to a 
 * Consumer.
 *
 * To test this function, run the binary and then echo "new data" >> testfile 
 */
fn run_tail_f() {
    let mut tail_f = Command::new("tail");
    tail_f.arg("-f");
    tail_f.arg("testfile.txt");
    let res = tail_f.status().expect("failed");
    println!();
=======
#![allow(unused)]
use clap::Parser;
use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;


/* Creating a test struct CLI to test command line arguments */
#[derive(Parser)]
struct CLi{
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    server_address: String,
    server_port: u16,
    log_file_path: String,
}

fn main() {
    let name = String::from("Security Log Collector");
    println!("Hello, {}",name);
    command_line();
    
>>>>>>> features
}

/* Testing CLI arguments into a textfile */
fn command_line() ->Result<()>{
    let args = CLi::parse();
    let path_name = &args.path;
    let content = match std::fs::read_to_string(path_name){
        Ok(content) => content,
        Err(error) => {
            println!("{}: {}", path_name.to_string_lossy(), error);
            return Err(error)
                .with_context(|| format!("could not read file `{}`", path_name.to_string_lossy()));
        }
    };
    
    println!("File Content: {}", content);
    let log_entries: Vec<Config> = serde_json::from_str(&content)
        .with_context(|| "failed to parse JSON data")?;

    println!("Log Entries:");
    for entry in &log_entries {
        println!("{:?}", entry);
    }
    Ok(())
}

