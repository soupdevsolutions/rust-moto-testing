use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use tokio;


const REGION: &str = "eu-west-1";

async fn build_client(endpoint_url: Option<&str>) -> aws_sdk_s3::Client {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let s3_config = aws_sdk_s3::config::Builder::from(&config);
    let s3_config = match endpoint_url {
        Some(endpoint_url) => s3_config.endpoint_url(endpoint_url),
        None => s3_config,
    };
    let s3_config = s3_config.region(Region::new(REGION)).build();

    let client = aws_sdk_s3::Client::from_conf(s3_config);

    client
}

async fn list_buckets(client: &aws_sdk_s3::Client) -> Vec<String> {
    let response = client
        .list_buckets()
        .send()
        .await
        .unwrap()
        .buckets
        .unwrap()
        .into_iter()
        .map(|b| b.name.unwrap())
        .collect();
    
    response 
}

async fn create_bucket(client: &aws_sdk_s3::Client, bucket_name: &str) {
    let constraint = BucketLocationConstraint::from(REGION);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    let _ = client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let client = build_client(None).await;

    let _create_result = create_bucket(&client, "test-bucket").await;
    let list_result = list_buckets(&client).await;

    println!("Buckets: ");
    for bucket in list_result {
        println!("\t Bucket: {}", bucket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_buckets() {
        let moto_client = build_client(Some("http://127.0.0.1:5000")).await;

        let result = list_buckets(&moto_client).await;
        assert!(result.len() == 0);

        let expected_buckets = 5;
        for i in 0..expected_buckets {
            create_bucket(&moto_client, &format!("test-bucket-{}", i)).await;
        }
        
        let result = list_buckets(&moto_client).await;
        assert!(result.len() == expected_buckets);
    }
}
