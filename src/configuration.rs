use std::env;

pub(crate) fn get_bucket_raw() -> String {
    let default_bucket_name = "raw-".to_owned() + &get_environment();
    env::var("BUCKET_RAW").unwrap_or_else(|_| default_bucket_name)
}

pub(crate) fn get_bucket_bronze() -> String {
    let default_bucket_name = "bronze-".to_owned() + &get_environment();
    env::var("BUCKET_BRONZE").unwrap_or_else(|_| default_bucket_name)
}

pub(crate) fn get_region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-west-1".to_string())
}

pub(crate) fn get_environment() -> String {
    env::var("ENVIRONMENT").unwrap_or_else(|_| "test".to_string())
}

pub(crate) fn get_localstack_endpoint() -> String {
    env::var("LOCALSTACK_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost.localstack.cloud:4566".to_string())
}

pub(crate) fn get_access_key() -> String {
    "ACCESS_KEY".to_string() // @NOTE: this should come from a secret manager
}

pub(crate) fn get_secret_key() -> String {
    "SECRET_KEY".to_string() // @NOTE: this should come from a secret manager
}
