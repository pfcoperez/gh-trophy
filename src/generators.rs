use crate::github::activity;
use crate::openscad::generators::generate_data_source;

use chrono::{Datelike, NaiveDate};

pub async fn generate_openscad(
    user_handle: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    maybe_token: Option<String>,
    maybe_static_code: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let result = activity::get_activity(&user_handle, (start_date, end_date), maybe_token).await?;

    let result_as_simple_matrix = result.as_matrix();
    let result_as_scad_data = generate_data_source(
        user_handle,
        format!(
            "{}/{} - {}/{}",
            start_date.year(),
            start_date.month(),
            end_date.year(),
            end_date.month()
        ),
        result_as_simple_matrix,
        maybe_static_code,
    );

    Ok(result_as_scad_data)
}
