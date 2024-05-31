provider "aws" {
  region = "us-west-2"
}

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


