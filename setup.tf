provider "aws" {
  region = "us-west-2"  
}

resource "aws_iam_user" "log_user" {
  name = "user"
}

resource "aws_iam_policy" "user_policy" {
  name        = "UserPolicy"
  path        = "/"
  description = "IAM policy for user"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action   = [
          "dynamodb:*",
          "cloudwatch:*"
        ]
        Effect   = "Allow"
        Resource = "*"
      }
    ]
  })
}

resource "aws_iam_user_policy_attachment" "user_policy_attachment" {
  user       = aws_iam_user.user.name
  policy_arn = aws_iam_policy.user_policy.arn
}

resource "aws_dynamodb_table" "security_logs" {
  name           = "SecurityLogs"
  billing_mode   = "PROVISIONED"
  hash_key       = "log_id"
  read_capacity  = 5
  write_capacity = 5

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
