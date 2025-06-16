use s3::{creds::Credentials, error::S3Error, Bucket, BucketConfiguration, Region};
use sea_orm::strum;
use tokio::fs::File;

pub struct PublicBucket {
    bucket: Result<Bucket, S3Error>,
}

#[derive(strum::IntoStaticStr)]
pub enum BucketNames {
    PUBLIC,
}

impl Default for PublicBucket {
    fn default() -> Self {
        let region = Region::Custom {
            region: "".to_string(),
            endpoint: "http://127.0.0.1:9000".to_string(), // Используем локальный адрес
        };

        let credentials =
            Credentials::new(Some("root-user"), Some("root-password"), None, None, None)
                .expect("Failed to create credentials");

        match Bucket::new("public", region, credentials) {
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
            println!("bucket: {:?}", bucket);
            let mut file_stream = File::open(fs_path).await?;
            bucket.put_object_stream(&mut file_stream, s3_path).await?;
            Ok(true)
        } else {
            println!("log2");
            let new_bucket = PublicBucket::create_bucket().await?;
            let mut file_stream = File::open(fs_path).await?;
            let bucket = new_bucket.bucket?;
            bucket.put_object_stream(&mut file_stream, s3_path).await?;
            Ok(true)
        }
    }

    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    async fn create_bucket() -> Result<Self, S3Error> {
        println!("log3");
        let bucket_name: &str = BucketNames::PUBLIC.into();
        let region = Region::UsEast1;
        let credentials = Credentials::default()?;
        let config = BucketConfiguration::default();
        let create_bucket_response =
            Bucket::create(bucket_name, region, credentials, config).await?;
        Ok(Self {
            bucket: Ok(*create_bucket_response.bucket),
        })
    }
}
