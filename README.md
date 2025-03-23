## Introduction

lambda-datafusion is a Rust project that implements an AWS Lambda function in Rust.

The lambda function listens for new Parquet files in an S3 bucket and executes SQL queries on the files using Apache DataFusion.
It is a PoC emulating data processing pipelines; where raw data land in the S3 bucker `raw-{ENV}` and the processed data land in the S3 bucket `bronze-{ENV}`.
Where `ENV` is the environment, e.g. `dev`, `test`, etc.
Additionally, some metrics are being sent to CloudWatch.

> DO NOT USE ANYWHERE; JUST FOR EDUCATIONAL PURPOSES<br />
❌ Horrible code quality <br />
❌ Barely any error handling <br />
❌ Very likely not be Rust-idiomatic

## Prerequisites

- Rust
- Cargo Lambda
- Apache DataFusion
- Localstack
- Terraform & terraform-local

## Building

```shell
  make build
```

## Deploy

```shell
  cd terraform
  tflocal apply -auto-approve -var-file=test.variables.tfvars
```
