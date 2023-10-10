extern crate maplit;

use maplit::hashmap;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, AttributeValue};
use rusoto_s3::{S3, S3Client, CreateBucketRequest};

pub async fn put_item_to_dynamodb() -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::PutItemError>> {
    let client = DynamoDbClient::new(Region::UsWest2);

    let item = hashmap! {
        "YourPrimaryKey".to_string() => AttributeValue {
            s: Some("YourValue".to_string()),
            ..Default::default()
        },
        // ... other attributes ...
    };

    let input = PutItemInput {
        table_name: "YourDynamoDBTable".to_string(),
        item,
        ..Default::default()
    };

    client.put_item(input).await?;

    Ok(())
}


pub async fn create_s3_bucket() -> Result<(), rusoto_core::RusotoError<rusoto_s3::CreateBucketError>> {
    let client = S3Client::new(Region::UsWest2);

    let request = CreateBucketRequest {
        bucket: "your-bucket-name".to_string(),
        create_bucket_configuration: Some(rusoto_s3::CreateBucketConfiguration {
            location_constraint: Some("us-west-2".to_string()),
        }),
        ..Default::default()
    };

    client.create_bucket(request).await?;

    Ok(())
}

