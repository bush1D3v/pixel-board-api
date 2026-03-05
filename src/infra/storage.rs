use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::{Client, config::Builder as S3ConfigBuilder};
use std::env;

pub struct Storage;

impl Storage {
    /// Creates an S3-compatible client configured for MinIO.
    ///
    /// Required env vars:
    /// - `MINIO_ENDPOINT` — e.g. `http://minio:9000`
    /// - `MINIO_ACCESS_KEY`
    /// - `MINIO_SECRET_KEY`
    /// - `MINIO_REGION` (optional, defaults to `us-east-1`)
    pub fn client() -> Client {
        let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");
        let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
        let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");
        let region = env::var("MINIO_REGION").unwrap_or_else(|_| "us-east-1".to_string());

        let credentials = Credentials::new(&access_key, &secret_key, None, None, "env");

        let config = S3ConfigBuilder::new()
            .endpoint_url(&endpoint)
            .region(Region::new(region))
            .credentials_provider(credentials)
            .force_path_style(true) // required for MinIO
            .behavior_version_latest()
            .build();

        Client::from_conf(config)
    }

    /// Ensures the bucket exists. Creates it if necessary.
    pub async fn ensure_bucket(client: &Client, bucket: &str) {
        let exists = client.head_bucket().bucket(bucket).send().await;
        if exists.is_err() {
            log::info!("Bucket '{bucket}' not found, creating...");
            client
                .create_bucket()
                .bucket(bucket)
                .send()
                .await
                .unwrap_or_else(|e| {
                    log::warn!("Could not create bucket '{bucket}': {e}");
                    panic!("Failed to create MinIO bucket '{bucket}'");
                });
            log::info!("Bucket '{bucket}' created successfully");
        }
    }
}
