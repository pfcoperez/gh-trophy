use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

type UserName<'a> = &'a str;

type DateRange = (NaiveDate, NaiveDate);

#[derive(Debug, Hash, Eq, PartialEq)]
struct YearWeek {
    year: usize,
    week: usize,
}

impl Serialize for YearWeek {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}-W{:02}", self.year, self.week))
    }
}

#[derive(Serialize, Debug)]
pub struct Activity {
    date_range: DateRange,
    contributions: HashMap<YearWeek, HashMap<Weekday, u32>>,
}

#[derive(Deserialize, Debug)]
struct Event {
    #[serde(rename = "type")]
    event_type: String,
    created_at: String,
}

fn get_year_week(date: NaiveDate) -> YearWeek {
    let iso_week = date.iso_week();
    YearWeek {
        year: iso_week.year() as usize,
        week: iso_week.week() as usize,
    }
}

pub async fn get_activity(
    user: UserName<'_>,
    date_range: DateRange,
) -> Result<Activity, Box<dyn std::error::Error>> {
    let number_of_weeks = ((date_range.1 - date_range.0).num_days() as f32 / 7.0).ceil() as usize;

    let mut contributions: HashMap<YearWeek, HashMap<Weekday, u32>> = HashMap::new();

    // Initialize the activity weeks
    for date in date_range.0.iter_weeks().take(number_of_weeks) {
        let year_week = get_year_week(date);
        contributions.insert(year_week, HashMap::new());
    }

    let client = reqwest::Client::new();

    // GitHub API endpoint for user events
    let url = format!("https://api.github.com/users/{}/events", user);

    // Make the request with required User-Agent header
    let response = client
        .get(&url)
        .header("User-Agent", "gh-trophy")
        .send()
        .await?;

    let events: Vec<Event> = response.json().await?;

    for event in events {
        if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(&event.created_at) {
            let date = datetime.naive_utc().date();

            if date >= date_range.0 && date <= date_range.1 {
                let year_week = get_year_week(date);
                let weekday = date.weekday();

                let entry = contributions.entry(year_week).or_insert_with(HashMap::new);
                *entry.entry(weekday).or_insert(0) += 1;
            }
        }
    }

    Ok(Activity {
        date_range,
        contributions,
    })
}
