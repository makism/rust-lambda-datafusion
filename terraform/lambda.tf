locals {
    lambda_name     = "lambda-datafusion"
}

resource "aws_lambda_function" "lambda_datafusion" {
  function_name   = local.lambda_name
  role            = aws_iam_role.lambda_exec.arn
  runtime         = "provided.al2023"
  handler         = "bootstrap"
  timeout         = 10
  memory_size     = 128
  architectures   = ["arm64"]

  environment {
    variables = {
      AWS_REGION      = var.region
      BUCKET_RAW      = aws_s3_bucket.bucket_raw.bucket
      BUCKET_BRONZE   = aws_s3_bucket.bucket_bronze.bucket
    }
  }

  filename         = "../target/lambda/${local.lambda_name}/bootstrap.zip"
  source_code_hash = filebase64sha256("../target/lambda/${local.lambda_name}/bootstrap.zip")

  tags = {
    Environment = var.environment
    Project     = "datafusion"
  }
}
