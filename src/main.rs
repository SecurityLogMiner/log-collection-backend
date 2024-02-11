mod config;
mod producer;
mod firehosesdk;
mod dynamosdk;
mod util;
mod iam;

use aws_config::imds::Client;
use producer::start_log_stream;
use config::read_config;
use std::env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        let config_data = read_config();


        match config_data {
            Some(config) => {
                if args.len() == 1 {

                //Create a setup functoin 
                // User gives IAM credentials; as long as they have correct policies; based on the policies set up on whatever they have available.
                // Attach policies to IAM user based on the set up function
                    let _ = start_log_stream(config.log_paths.clone()).await;
                }
                if args.len() == 2 {
                    if args[1] == "--help" || args[1] == "-h" {
                        util::print_help().await;
                    }

                    let destination = args[1].as_str();
                    println!("Destination: {}", destination);
                    match destination {
                        "dynamodb" => {
                            util::send_dynamodb(config).await;
                        }
                        "elastic" => {
                            todo!();
                        }
                        "iam" => {
                            util::initialize_iam(config).await;
                            // let output = Command::new("aws")
                            //     .args(&["iam", "list-users"])
                            //     .output()
                            //     .expect("Failed to execute command");

                            // println!("IAM Policies: {}", String::from_utf8_lossy(&output.stdout));
                        }
                        "run-admin" => {
                            // util::initialize_iam(config).await;
                            util::run_admin_cli().await;
                        }

                        _ => {
                            util::print_help().await;
                        }
                    }
                }
            }
            None => panic!("Error reading configuration. Fix it."),
        }
    } 

    Ok(())
}

