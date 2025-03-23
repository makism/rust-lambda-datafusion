use crate::configuration::{get_access_key, get_localstack_endpoint, get_region, get_secret_key};
use crate::metrics::send_metric_to_cloudwatch;
use aws_lambda_events::s3::S3Object;
use aws_sdk_cloudwatch::types::StandardUnit;
use datafusion::common::ScalarValue;
use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::file_format::FileFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::prelude::{SessionConfig, SessionContext};
use lambda_runtime::Error;
use object_store::aws::AmazonS3Builder;
use std::sync::Arc;
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;
use url::Url;

async fn register_s3_object_store(
    ctx: &datafusion::prelude::SessionContext,
    bucket_name: &str,
) -> datafusion::common::Result<()> {
    let s3 = AmazonS3Builder::new()
        .with_endpoint(get_localstack_endpoint())
        .with_bucket_name(bucket_name.clone())
        .with_region(get_region())
        .with_access_key_id(get_access_key())
        .with_secret_access_key(get_secret_key())
        .with_allow_http(true)
        .with_skip_signature(true)
        .build()?;

    let path = format!("s3://{bucket_name}");
    let s3_url = Url::parse(&path).unwrap();
    let arc_s3 = Arc::new(s3);

    let obj_store = ctx.register_object_store(&s3_url, arc_s3.clone());
    if obj_store.is_some() {
        return Err(datafusion::error::DataFusionError::Execution(
            "Failed to register object store".to_string(),
        ));
    }

    Ok(())
}

pub(crate) async fn pipeline(
    bucket_raw: String,
    bucket_bronze: String,
    object: S3Object,
) -> datafusion::common::Result<(), Error> {
    let retry_strategy = ExponentialBackoff::from_millis(100).map(jitter).take(3);

    let result_session= Retry::spawn(retry_strategy.clone(), || {
        prepare_session(&bucket_raw, &bucket_bronze, object.clone())
    })
    .await;

    let ctx = result_session.unwrap_or_else(|e| {
        panic!("Failed to prepare session: {:?}", e);
    });

    let result_data= Retry::spawn(retry_strategy.clone(), || {
        process_data(&ctx, &bucket_bronze)
    })
    .await;

    Ok(())
}

async fn prepare_session(
    bucket_raw: &String,
    bucket_bronze: &String,
    object: S3Object,
) -> Result<SessionContext, Error> {
    let config = SessionConfig::new()
        .set(
            "datafusion.execution.batch_size",
            &ScalarValue::UInt64(Some(100)),
        )
        .set_bool("datafusion.execution.parquet.pushdown_filters", true);

    let ctx = datafusion::prelude::SessionContext::new_with_config(config);

    let _ = register_s3_object_store(&ctx, &bucket_raw).await;
    let _ = register_s3_object_store(&ctx, &bucket_bronze).await;

    let file_input = object.key.unwrap();

    let path = format!("s3://{bucket_raw}/{file_input}");
    let file_format = ParquetFormat::default().with_enable_pruning(true);

    let listing_options = ListingOptions::new(Arc::new(file_format))
        .with_file_extension(ParquetFormat::default().get_ext());

    let _ = ctx
        .register_listing_table("raw", &path, listing_options, None, None)
        .await;

    Ok(ctx)
}

async fn process_data(
    ctx: &SessionContext,
    bucket_bronze: &String,
) -> datafusion::common::Result<()> {
    let df = ctx.sql("SELECT * from raw").await?;

    let result_num_rows = df.clone().count();
    let num_rows = result_num_rows.await.unwrap_or_else(
        |e| {
            panic!("Failed to count rows: {:?}", e);
        },
    );

    send_metric_to_cloudwatch(
        "raw-data-processing",
        "num_rows",
        num_rows as f64,
        StandardUnit::Count,
    )
    .await;

    let target = format!("s3://{bucket_bronze}/");
    let options =
        DataFrameWriteOptions::new().with_partition_by(vec!["Partition_Date".to_string()]);

    let result_write = df.clone().write_parquet(&target, options, None).await;
    let _ = result_write.unwrap_or_else(|e| {
        panic!("Failed to write parquet: {:?}", e);
    });

    Ok(())
}
