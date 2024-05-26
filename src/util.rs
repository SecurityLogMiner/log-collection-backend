use aws_sdk_dynamodb::client;

// util.rs serves as housing various utility functions that are used in main.rs
use crate::config::Config;
use crate::producer::start_log_service;
use crate::dynamosdk; // Import other modules as needed
use std::process;
use std::fmt;
use crate::iam;
use aws_sdk_iam::types::User;

pub struct UserDisplay<'a>(pub &'a aws_sdk_iam::types::User);


pub async fn print_help() {
    println!("Usage: cargo run -- <destination>");
    println!("Available Destinations:");
    println!("  all            Send logs to all locations");
    println!("  dynamodb       Create DynamoDB table");
    println!("  kdf            Send logs to Kinesis Firehose");
    println!("  s3             Send logs to S3 bucket");
    println!("  iam            Test iam features");
    println!("  run-admin      Connecto to the Administrator AWS CLI");
    println!("  elastic        Send logs to Elastic");
    process::exit(0);
}


impl<'a> fmt::Display for UserDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let user: &User = self.0;
        write!(f,
            "Username: {}\nUser ID: {}\nARN: {}\nPermission Boundaries: {:?}\n",
            user.user_name, user.user_id, user.arn, user.permissions_boundary)
        
    }    
}

pub async fn initialize_iam(config:Config){

    let iam_client = iam::start_iam().await;
    println!("{:?}",&config);
    match iam_client {
        Ok(client) => {
            // Check if the user exists
            // List all the current users; must require IAM policy
            // Currently endpoint users are able to list this out along with admins but this is not advisable. 
            // I'm sure there is a policy on iam to have them list only thier own credentials and users
            println!("\nListing all current users");
            let users = iam::list_users(&client, None, None, None)
            .await
            .unwrap();
            for user in users.users {
                println!("{}", user.user_name);
            }
            
            if let Ok(user) = iam::get_user(&client).await {
                println!("\nCurrent User:");
                println!("{}", UserDisplay(user.user.as_ref().unwrap()));            
            } 
            else {
                eprintln!("Failed to get the user. Please check your network connection and IAM permissions, and try again.");
            }
        }
               
            Err(err) => {
                println!("Error occurred starting IAM client: {}", err);
            }
    }
    
}

