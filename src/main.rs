mod github;

use tokio;
use crate::github::activity;

#[tokio::main]
async fn main() {
    activity::get_activity("example_user").await;
    println!("Hello, world!");
}