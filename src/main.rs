use gh_trophy::generators::generate_openscad;
use gh_trophy::openscad::resources::trophy_without_data;

use tokio;

/// CLI tool to download user activity data from GitHub
/// and represent it as OpenSCAD source files that can be
/// included in 3D models represented in this language.
/// Expects one parameter with the GitHub user profile handle.
/// It will obtain activity data over the last year to the date
/// (From today-365 days to today).
/// If the `GITHUB_TOKEN` environment variable is present, it will
/// use it an application token to authenticate with GitHub API,
/// this makes it possible to include private repositories activity
/// on the response.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maybe_user_handle = std::env::args().nth(1);

    let end_date = chrono::Utc::now().naive_utc().date();
    let start_date = end_date - chrono::Duration::days(365);

    if let Some(user_handle) = maybe_user_handle {
        let maybe_token = std::env::var("GITHUB_TOKEN").ok();

        let result_as_scad_data = generate_openscad(
            user_handle,
            start_date,
            end_date,
            maybe_token,
            Some(trophy_without_data()),
        )
        .await?;

        println!("{}", result_as_scad_data);
        return Ok(());
    } else {
        let error_msg = "Please provide a GitHub user handle as the first argument";
        eprintln!("{}", error_msg);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, error_msg).into());
    }
}
