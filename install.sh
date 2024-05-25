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

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Check the Rust installation
rustc --version

# Print success message
echo "Rust installed successfully."

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

