mod github;
mod openscad;

use crate::github::activity;
use crate::openscad::generators::generate_matrix_source;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maybe_token = std::env::var("GITHUB_TOKEN").ok();

    // Define a date range for the last 30 days
    let end_date = chrono::Utc::now().naive_utc().date();
    let start_date = end_date - chrono::Duration::days(360);

    let result = activity::get_activity("pfcoperez", (start_date, end_date), maybe_token).await?;

    let result_as_simple_matrix = result.as_matrix();
    let result_as_scad_data =
        generate_matrix_source("rawActivity".to_string(), result_as_simple_matrix);
    println!("{}", result_as_scad_data);

    println!("Successfully fetched activity for user");

    Ok(())
}
