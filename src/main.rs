mod github;

use crate::github::activity;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maybe_token = std::env::var("GITHUB_TOKEN").ok();

    // Define a date range for the last 30 days
    let end_date = chrono::Utc::now().naive_utc().date();
    let start_date = end_date - chrono::Duration::days(360);

    let result = activity::get_activity("pfcoperez", (start_date, end_date), maybe_token).await?;

    let result_as_json = serde_json::to_string_pretty(&result)?;
    println!("{}", result_as_json);

    println!("Successfully fetched activity for user");

    Ok(())
}
