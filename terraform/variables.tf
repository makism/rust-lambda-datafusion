variable "environment" {
  description = "The environment for the Lambda function (e.g., dev, prod)"
  type        = string
  default     = "dev"
}

variable "region" {
    description = "The AWS region where the resources will be created"
    type        = string
    default     = "eu-west-1"
}

variable "bucket_force_destroy" {
    description = "A boolean flag to enable the deletion of non-empty S3 buckets"
    type        = bool
    default     = false
}
