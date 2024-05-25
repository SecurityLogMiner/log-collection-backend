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

    # Source the cargo environment script
    echo "Adding Rust to the shell environment..."
    source $HOME/.cargo/env

fi