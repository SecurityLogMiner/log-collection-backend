#!/bin/bash

# Define the log directory path
LOG_DIR="/var/log/logminer/logs"
# TODO - add the setup to add user permissions to interact with the log directory
# setfacl -m u:username:perms /path/to/log/file

# Create the directory if it doesn't exist
if [ ! -d "$LOG_DIR" ]; then
    mkdir -p "$LOG_DIR"
    chmod 700 "$LOG_DIR"  # Adjust permissions as needed
    echo "Log directory created at: $LOG_DIR"
else
    echo "Log directory already exists at: $LOG_DIR"
fi

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

    # Add Rust to the shell initialization file for future sessions
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        SHELL_RC="$HOME/.bashrc"
    else
        SHELL_RC="$HOME/.profile"
    fi
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> $SHELL_RC
fi