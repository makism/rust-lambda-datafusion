use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cloudwatch::types::{MetricDatum, StandardUnit};
use aws_sdk_cloudwatch::Client;
use tokio;

pub(crate) async fn get_cloudwatch_client() -> Client {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

pub(crate) async fn send_metric_to_cloudwatch(
    namespace: &str,
    metric_name: &str,
    value: f64,
    unit: StandardUnit,
) -> () {
    let client = get_cloudwatch_client().await;

    let metric = MetricDatum::builder()
        .metric_name(metric_name)
        .value(value)
        .unit(unit)
        .build();

    let _ = client
        .put_metric_data()
        .namespace(namespace)
        .metric_data(metric)
        .send()
        .await;

    println!("Metric sent: {} = {}", metric_name, value);
}
