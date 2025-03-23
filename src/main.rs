use lambda_runtime::{run, service_fn, Error};
mod configuration;
mod dataproc;
mod event_handler;
mod metrics;

use event_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let func = service_fn(|event| function_handler(event));
    run(func).await
}
