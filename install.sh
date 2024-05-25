#!/bin/bash

# Define the log directory path
LOG_DIR="/logs"
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
