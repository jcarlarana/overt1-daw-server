mod handlers;
mod models;
mod utils;

use crate::utils::aws_utils;

#[tokio::main]
async fn main() {
    let routes = handlers::get_routes();
    aws_utils::put_item_to_dynamodb().await.unwrap();
    aws_utils::create_s3_bucket().await.unwrap();
}

