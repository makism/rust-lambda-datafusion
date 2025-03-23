locals {
  bucket_name_raw    = "raw"
  bucket_name_bronze = "bronze"
}

resource "aws_s3_bucket" "bucket_raw" {
  bucket = "${local.bucket_name_raw}-${var.environment}"

  force_destroy = var.bucket_force_destroy

  tags = {
    Environment = var.environment
    Project     = "datafusion"
  }
}

resource "aws_s3_bucket" "bucket_bronze" {
  bucket = "${local.bucket_name_bronze}-${var.environment}"

  force_destroy = var.bucket_force_destroy

  tags = {
    Environment = var.environment
    Project     = "datafusion"
  }
}

resource "aws_s3_bucket_notification" "bucket_notification" {
  bucket = "${local.bucket_name_raw}-${var.environment}"

  lambda_function {
    lambda_function_arn = aws_lambda_function.lambda_datafusion.arn
    events              = ["s3:ObjectCreated:*"]
  }
}