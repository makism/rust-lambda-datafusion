use crate::configuration::{get_bucket_bronze, get_bucket_raw};
use crate::dataproc::pipeline;
use aws_lambda_events::event::s3::S3Event;
use aws_lambda_events::s3::object_lambda::SessionContext;
use aws_lambda_events::s3::{S3Bucket, S3Entity, S3EventRecord, S3Object};
use aws_sdk_s3::types::Bucket;
use aws_sdk_s3::Client as S3Client;
use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::file_format::FileFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::error::Result;
use datafusion::functions_aggregate::expr_fn::min;
use datafusion::prelude::*;
use datafusion::scalar::ScalarValue;
use lambda_runtime::tracing::warn;
use lambda_runtime::{tracing, Error, LambdaEvent};
use object_store::aws::AmazonS3Builder;
use std::env;
use std::sync::Arc;
use tracing::{trace, warn_span};
use url::Url;

pub(crate) async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    tracing::info!("Handler starts");

    let payload = event.payload;
    if payload.records.len() == 0 {
        tracing::info!("Empty S3 event received");
    }

    for record in payload.records {
        if let Some(event_name) = record.event_name.as_deref() {
            match event_name {
                "ObjectCreated:Put" => {
                    let key = record.s3.object;

                    let bucket_raw = get_bucket_raw();
                    let bucket_bronze = get_bucket_bronze();

                    tracing::info!("BUCKET RAW: {}", bucket_raw);
                    tracing::info!("BUCKET BRONZE: {}", bucket_bronze);

                    pipeline(bucket_raw, bucket_bronze, key).await?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
