use gh_trophy::generators::generate_openscad;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maybe_user_handle = std::env::args().nth(1);

    let end_date = chrono::Utc::now().naive_utc().date();
    let start_date = end_date - chrono::Duration::days(360);

    if let Some(user_handle) = maybe_user_handle {
        let maybe_token = std::env::var("GITHUB_TOKEN").ok();

        let result_as_scad_data =
            generate_openscad(user_handle, start_date, end_date, maybe_token).await?;

        println!("{}", result_as_scad_data);
        return Ok(());
    } else {
        let error_msg = "Please provide a GitHub user handle as the first argument";
        eprintln!("{}", error_msg);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, error_msg).into());
    }
}
