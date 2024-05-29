<a href="top"></a>
# Table of Contents
1. [Overview](#overview)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Contributing](#contributing)
5. [License](#license)

## Overview
The Log Collection system offers a method of gathering logs from an endpoint and storing those logs on a centralized server.
The system stores these logs using AWS resources such as DynamoDB and IAM for storage and credentials.
## Installation

### Prerequisutes
The Log Collection client requires:
- Amazon Web Services (AWS) CLI
- Rust
- Terraform

Ensure that Rust and AWS CLI are installed and configured on your machine. You can run the install bash script to 
configure and install dependencies.
```
./install.sh
```
The bash install script will install Rust and AWS CLI on the local machine.
The logs will stored in a newly created directory called ./logs. 
For testing purposes, three fake logs are generated.

# Setting Up Your Rust Environment

To ensure that your Rust environment is correctly configured, you need to source the `$HOME/.cargo/env` file on Linux or set up the equivalent on Windows. Follow the appropriate steps for your operating system below:

## Linux

1. **Open your terminal.**

2. **Source the environment file** by running the following command:

    ```sh
    source $HOME/.cargo/env
    ```

    This command updates your current terminal session to include the Rust binaries in your PATH. You will need to run this command every time you open a new terminal session unless you add it to your shell's startup file (e.g., `.bashrc`, `.zshrc`, etc.).

3. **Optional: Add to your shell startup file**

    To automatically source the Rust environment every time you start a new terminal session, add the following line to your shell's startup file:

    ```sh
    echo 'source $HOME/.cargo/env' >> ~/.bashrc
    ```

    If you use a different shell, replace `~/.bashrc` with the appropriate configuration file (e.g., `~/.zshrc` for Zsh).

## Windows

1. **Open PowerShell** as Administrator.

2. **Set the environment variable for the current session** by running the following command:

    ```powershell
    $env:PATH += ";$env:USERPROFILE\.cargo\bin"
    ```

    This command updates your current PowerShell session to include the Rust binaries in your PATH. You will need to run this command every time you open a new PowerShell session unless you add it permanently.

3. **Optional: Add to your system PATH permanently**

    To permanently add Rust to your system PATH, follow these steps:

    1. Open the Start Menu and search for "Environment Variables."
    2. Click on "Edit the system environment variables."
    3. In the System Properties window, click on the "Environment Variables" button.
    4. In the Environment Variables window, under "User variables" or "System variables," find and select the `Path` variable, then click "Edit."
    5. In the Edit Environment Variable window, click "New" and add the following path:

        ```
        %USERPROFILE%\.cargo\bin
        ```

    6. Click "OK" to close all the windows.

After completing these steps, your Rust environment should be properly configured for your operating system.

## Usage

## Configuration
Create a new file named _config.toml_ and specify the logs and table names on AWS DynamoDB.
```
# config.toml
[dynamodb]

[[dynamodb.package]]
source = "test1.log"
table = "SecurityLog1"

[[dynamodb.package]]
source = "test2.log"
table = "SecurityLog2"

[[dynamodb.package]]
source = "test3.log"
table = "SecurityLog3"
```
This configuration file specifies three sources test1, test2, test3, and stores them into their respective tables.


### Running the Log Collection System
1. Start the log collector through Rust
```
cargo build
cargo run
```
2. Menu Options:
    Send Logs: Enter the path to the logs directory to start sending logs to DynamoDB.
    Stop Logs: Stop the log collection process gracefully.
    View Logs: View the logs that are currently being collected.
    Exit: Exit the menu.

Specifying "Send Logs" will prompt the log collector to continuously monitor log behavior in the background and append these updates to AWS DynamoDB to their specified tables.

## Contributing
We welcome contributions! Please submit a [new issue](https://github.com/SecurityLogMiner/log-collection-client/issues/new) to improve the log collection client!

## License
Apache 2.0


