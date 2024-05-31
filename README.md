<a href="top"></a>
# Table of Contents
1. [Overview](#overview)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Contributing](#contributing)
5. [License](#license)
6. [Feedback](#feedback)

## Overview
System Management Software provides the insight needed to secure, troubleshoot, and optimize systems and applications. Whether it is an individual user or larger organization, log collection is the first step in the analysis process. The collection and storage of system and application logs is designed with ease-of-use in mind to provide simple and efficient event visibiilty for any device. 

## Installation

### Prerequisutes
The Log Collection client requires:
- Amazon Web Services (AWS) CLI
- Rust
- Terraform
- Curl
- unzip
- wget


Ensure that Rust and AWS CLI is installed and configured on your machine. You can run the install bash script to 
configure and install dependencies.
```
./install.sh
```

## Usage
=======
- [Getting Started](#getting-started)
- [Resources](#resources)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)

### Creating Issues
TODO

## Getting Started
Create an AWS account, setup IAM and bucket policies.
1. [Create an AWS Account](https://portal.aws.amazon.com/billing/signup#/start/email)

2. Set up Identity and Access Management account (IAM).
    - Note: Be sure to copy down your access and secret access key and save them locally.

Clone the client repositories to start.
- [Client](https://github.com/SecurityLogMiner/log-collection-client)

The client will read the configuration file and begin processing and sending 
log data from the given PATH to the server.

3. Run the install script to create logs and the directory to store them in, as well as installing terraform, AWSCLI, and Rust.

install.sh:
```
#!/bin/bash

# Define the log directory path
LOG_DIR="./logs"
LOG_PREFIX="test"
MAX_LOGS=3

# Create the log directory if it doesn't exist
if [ ! -d "$LOG_DIR" ]; then
    mkdir -p "$LOG_DIR"
    chmod 700 "$LOG_DIR"  # Adjust permissions as needed
    echo "Log directory created at: $LOG_DIR"
else
    echo "Log directory already exists at: $LOG_DIR"
fi

# TODO - add the setup to add user permissions to interact with the log directory
# Replace 'username' with the actual username that needs permissions
# setfacl -m u:username:rwx "$LOG_DIR"


# Function to generate a random security log entry
generate_log_entry() {
  TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
  EVENT_ID=$((RANDOM % 1000 + 1000))
  SOURCE="Security"
  MESSAGE="Random security log message with ID $EVENT_ID"
  echo "$TIMESTAMP - $SOURCE - Event ID: $EVENT_ID - $MESSAGE"
}

# Create and write to the log files
for ((i=1; i<=MAX_LOGS; i++)); do
  LOG_FILE="${LOG_DIR}/${LOG_PREFIX}${i}.log"
  LOG_ENTRY=$(generate_log_entry)
  echo "$LOG_ENTRY" > "$LOG_FILE"
done

echo "Log files created successfully."

# Check if Terraform is already installed

if [ ! -f "/usr/local/bin/terraform" ]; then

    # Set the desired Terraform version
    TERRAFORM_VERSION="1.2.9"

    # Download Terraform. Adjust the version number as necessary.
    wget https://releases.hashicorp.com/terraform/${TERRAFORM_VERSION}/terraform_${TERRAFORM_VERSION}_linux_amd64.zip

    # Unzip the downloaded file
    unzip terraform_${TERRAFORM_VERSION}_linux_amd64.zip

    # Move the executable to a directory included in the system's PATH
    sudo mv terraform /usr/local/bin/

    # Remove the downloaded ZIP file
    rm terraform_${TERRAFORM_VERSION}_linux_amd64.zip

    # Check the installation
    terraform --version

    # Print success message
    echo "Terraform installed successfully."

else
    echo "Terraform is already installed."
    terraform --version
fi

# Check if AWS CLI is already installed
if command -v aws >/dev/null 2>&1; then
    echo "AWS CLI is already installed."
    aws --version
else
    echo "AWS CLI is not installed. Proceeding with installation..."

    curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
    unzip awscliv2.zip
    sudo ./aws/install
    rm -rf awscliv2.zip aws/

fi

# Check if Rust is already installed
if command -v rustc >/dev/null 2>&1; then
    echo "Rust is already installed."
    rustc --version
    cargo --version
else
    echo "Rust is not installed. Proceeding with installation..."
    # Install required dependencies
    sudo apt install -y curl build-essential
    # Download and run the Rust installation script
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y


fi
```


### Running the Log Collection System
1. Start the log collector through Rust
```
cargo build
[default] 
  aws_access_key_id=YOUR-ACCESS-KEY
  aws_secret_access_key=YOUR-SECRET-KEY
```
Enter the command "aws config" to configure these credentials as well as the output type(text) and region "us-west-2".

The client will look for these credentials when executed.

4. Use command "terraform init" and then "terraform apply" to create user profile under the users IAM group. This gives the proper user permissions and gives Cloud Watch resource access.

setup.tf:
```
# Generate a random 4-digit number
resource "random_id" "user_id" {
  byte_length = 2
  keepers = {
    # Ensure a new ID is generated when any input variable changes
    always_run = "${timestamp()}"
  }
}

provider "aws" {
  region = "us-west-2"
  shared_credentials_files = ["~/.aws/credentials"]
  shared_config_files      = ["~/.aws/config"]
  profile                  = "default"
}

# Create IAM user with random 4-digit ID
resource "aws_iam_user" "log_user" {
  name = "user${random_id.user_id.hex}"
}

# Add the user to the group 'users'
resource "aws_iam_user_group_membership" "log_user_group_membership" {
  user   = aws_iam_user.log_user.name
  groups = ["users"]
}

# Define other resources as needed
resource "aws_cloudwatch_log_group" "security_logs" {
  name = "/aws/rust/logs"
}

resource "aws_cloudwatch_log_stream" "security_log_stream" {
  name           = "LogStream"
  log_group_name = aws_cloudwatch_log_group.security_logs.name
}
```

Running the Client:
```
cd <client_repo_dir>
cargo install
cargo run
```
2. Menu Options:
    Send Logs: Enter the path to the logs directory to start sending logs to DynamoDB.
    Stop Logs: Stop the log collection process gracefully.
    View Logs: View the logs that are currently being collected.
    Exit: Exit the menu.

## Contributing
We welcome contributions! Please submit a [new issue](https://github.com/SecurityLogMiner/log-collection-client/issues/new) to improve the log collection client!

## License
Apache 2.0

## Feedback
We would love to hear your thoughts and suggestions. Please open an issue on Github or contact us at [logcollectionsystem@gmail.com](logcollectionsystem@gmail.com)

