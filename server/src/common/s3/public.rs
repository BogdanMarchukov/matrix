use s3::{creds::Credentials, error::S3Error, Bucket, BucketConfiguration, Region};
use sea_orm::strum;
use tokio::fs::File;

use crate::config::S3Config;

pub struct PublicBucket {
    bucket: Result<Bucket, S3Error>,
}

#[derive(strum::IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
enum BucketNames {
    PUBLIC,
}

impl Default for PublicBucket {
    fn default() -> Self {
        let s3_config = S3Config::new();
        let region = Region::Custom {
            region: "".to_string(),
            endpoint: s3_config.endpoint,
        };

        let credentials = Credentials::new(
            Some(&s3_config.root_user),
            Some(&s3_config.root_password),
            None,
            None,
            None,
        )
        .expect("Failed to create credentials");

        match Bucket::new(BucketNames::PUBLIC.into(), region, credentials) {
            Ok(bucket) => Self {
                bucket: Ok(*bucket.with_path_style()),
            },
            Err(err) => {
                eprintln!("Bucket creation error: {:?}", err);
                Self { bucket: Err(err) }
            }
        }
    }
}

impl PublicBucket {
    pub async fn put_object(fs_path: &String, s3_path: &String) -> Result<bool, S3Error> {
        if let Ok(bucket) = PublicBucket::new().bucket {
            let mut file_stream = File::open(fs_path).await?;
            match bucket.put_object_stream(&mut file_stream, s3_path).await {
                Ok(_) => Ok(true),
                Err(_) => {
                    let new_bucket = PublicBucket::create_bucket().await?;
                    let mut file_stream = File::open(fs_path).await?;
                    let bucket = new_bucket.bucket?;
                    bucket.put_object_stream(&mut file_stream, s3_path).await?;
                    Ok(true)
                }
            }
        } else {
            let new_bucket = PublicBucket::create_bucket().await?;
            let mut file_stream = File::open(fs_path).await?;
            let bucket = new_bucket.bucket?;
            bucket.put_object_stream(&mut file_stream, s3_path).await?;
            Ok(true)
        }
    }

    pub fn get_name() -> String {
        let name: &str = BucketNames::PUBLIC.into();
        name.to_string()
    }

    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    async fn create_bucket() -> Result<Self, S3Error> {
        let s3_config = S3Config::new();
        let region = Region::Custom {
            region: "".to_string(),
            endpoint: s3_config.endpoint,
        };

        let credentials = Credentials::new(
            Some(&s3_config.root_user),
            Some(&s3_config.root_password),
            None,
            None,
            None,
        )?;
        let config = BucketConfiguration::public();
        let create_bucket_response =
            Bucket::create_with_path_style(BucketNames::PUBLIC.into(), region, credentials, config)
                .await?;
        Ok(Self {
            bucket: Ok(*create_bucket_response.bucket),
        })
    }
}
