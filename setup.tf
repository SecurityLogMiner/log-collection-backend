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

# Create IAM user with random 4-digit ID
resource "aws_iam_user" "log_user" {
  name = "user${random_id.user_id.hex}"
}

# Attach AmazonDynamoDBReadOnlyAccess policy to the user
resource "aws_iam_user_policy_attachment" "dynamodb_readonly" {
  user       = aws_iam_user.log_user.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonDynamoDBReadOnlyAccess"
}

# Attach IAMReadOnlyAccess policy to the user
resource "aws_iam_user_policy_attachment" "iam_readonly" {
  user       = aws_iam_user.log_user.name
  policy_arn = "arn:aws:iam::aws:policy/IAMReadOnlyAccess"
}

# Define other resources as needed
resource "aws_dynamodb_table" "security_logs" {
  name           = "SecurityLogs"
  billing_mode   = "PROVISIONED"
  hash_key       = "log_id"
  read_capacity  = 5
  write_capacity  = 5

  attribute {
    name = "log_id"
    type = "S"
  }
}

resource "aws_cloudwatch_log_group" "security_logs" {
  name = "/aws/rust/logs"
}

resource "aws_cloudwatch_log_stream" "security_log_stream" {
  name           = "LogStream"
  log_group_name = aws_cloudwatch_log_group.security_logs.name
}

resource "null_resource" "install_rust" {
  # Triggers the installation only when the variable changes
  triggers = {
    always_run = "${timestamp()}"
  }

  provisioner "local-exec" {
    command = <<EOF
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source $HOME/.cargo/env
    EOF
    interpreter = ["/bin/bash", "-c"]
  }
}
