use aws_sdk_iam::operation::get_user::{GetUserError, GetUserOutput};
use aws_sdk_iam::{Client, Error};
use aws_sdk_iam::operation::{
    list_users::*, list_groups_for_user::*,
};
// use aws_sdk_iam::model::{ListUserPoliciesInput, ListUserPoliciesOutput};

use std::{env, process::Command};
use std::io::{self, Write};
use aws_sdk_iam::error::SdkError;
use tokio::time::{sleep, Duration};
use aws_sdk_iam::types::{AccessKey, Policy, PolicyScopeType, Role, User};


// #[::tokio::main]


pub async fn
start_iam() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_iam::Client::new(&config);
    Ok(client)
}

pub async fn list_users(
    client: &Client,
    path_prefix: Option<String>,
    marker: Option<String>,
    max_items: Option<i32>,
) -> Result<ListUsersOutput, SdkError<ListUsersError>> {
    let response = client
        .list_users()
        .set_path_prefix(path_prefix)
        .set_marker(marker)
        .set_max_items(max_items)
        .send()
        .await?;
    Ok(response)
}

pub async fn list_groups_for_user(
    client: &Client,
    user_name: String,
    marker: Option<String>,
) -> Result<ListGroupsForUserOutput, SdkError<ListGroupsForUserError>> {
    let response = client
        .list_groups_for_user()
        .set_user_name(Some(user_name))
        .set_marker(marker)
        .send()
        .await?;
    Ok(response)
}


pub async fn is_admin_user(user: &User, iam_client: &Client) -> bool {
    let user_name = user.user_name.clone(); 

    let result = list_groups_for_user(iam_client, user_name.clone(), None).await;
    println!("Result: {:?}", result);
    match result {
        Ok(output) => {
            println!("User groups:");
            for group in &output.groups {
                let group_name = &group.group_name;
                    println!("{}", group_name);
                    if group_name == "admin" {
                        return true;
                    }
                
            }
            false // Return false if the user is not in the admin group
        },
        Err(err) => {
            eprintln!("Error occurred while checking user groups: {:?}", err);
            false // Return false if an error occurs
        }
    }
}


pub async fn get_user(
    client: &Client,
) -> Result<GetUserOutput, SdkError<GetUserError>> {
    let response: GetUserOutput = client.get_user().send().await?;
    Ok(response)
}


// Asynchronously runs the Administrator AWS CLI if the user is an admin
pub async fn run_admin_cli() -> Result<(), Error> { 

    let iam_client = start_iam().await;
    match iam_client {
        Ok(client) => {
            let user = get_user(&client).await.unwrap();
            if is_admin_user(&user.user.as_ref().unwrap(), &client).await {
                let stdin = io::stdin();
                let mut stdout = io::stdout();
        
                println!("\nRunning Administrator AWS CLI:");
                loop {
                    print!("aws> ");
                    stdout.flush().unwrap();
                    let mut input = String::new();
                    stdin.read_line(&mut input).unwrap();
                    if input.trim().eq_ignore_ascii_case("exit") {
                        break;
                    }
                    let args: Vec<&str> = input.trim().split_whitespace().collect();
                    if args.is_empty() {
                        continue;
                    }
        
                    let output = Command::new("aws")
                        .args(&args)
                        .output()
                        .expect("Failed to execute command");
        
                    io::stdout().write_all(&output.stdout).unwrap();
                    io::stderr().write_all(&output.stderr).unwrap();
                } 
            }
            else{
                println!("\n{} is not an admininstrator!", user.user.as_ref().unwrap().user_name);
                println!("Please contact the admininistrator for more information.");
            }
        }
        Err(err) => {
            println!("Error occurred starting IAM client: {}", err);
        }
    }
    Ok(())
}

