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

## Contributing
We welcome contributions! Please submit a [new issue](https://github.com/SecurityLogMiner/log-collection-client/issues/new) to improve the log collection client!

## License
Apache 2.0

## Feedback
We would love to hear your thoughts and suggestions. Please open an issue on Github or contact us at [logcollectionsystem@gmail.com](logcollectionsystem@gmail.com)

