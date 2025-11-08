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

impl Activity {
    pub fn number_of_weeks(&self) -> usize {
        ((self.date_range.1 - self.date_range.0).num_days() as f32 / 7.0).ceil() as usize
    }

    pub fn as_matrix(&self) -> Vec<Vec<u32>> {
        let mut matrix: Vec<Vec<u32>> = vec![vec![0; 7]; self.number_of_weeks()];
        for date in self.date_range.0.iter_weeks().take(self.number_of_weeks()) {
            let year_week = get_year_week(date);
            if let Some(week_entry) = self.contributions.get(&year_week) {
                for (weekday, count) in week_entry {
                    let week_index = year_week.week - 1;
                    let day_index = weekday.num_days_from_monday() as usize;
                    matrix[week_index][day_index] = count.clone();
                }
            }
        }
        return matrix;
    }
}

// GraphQL request and response structures
#[derive(Serialize, Debug)]
struct GraphQLRequest {
    query: String,
    variables: GraphQLVariables,
}

#[derive(Serialize, Debug)]
struct GraphQLVariables {
    username: String,
    from: String,
    to: String,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse {
    data: GraphQLData,
}

#[derive(Deserialize, Debug)]
struct GraphQLData {
    user: User,
}

#[derive(Deserialize, Debug)]
struct User {
    #[serde(rename = "contributionsCollection")]
    contributions_collection: ContributionsCollection,
}

#[derive(Deserialize, Debug)]
struct ContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    contribution_calendar: ContributionCalendar,
}

#[derive(Deserialize, Debug)]
struct ContributionCalendar {
    #[serde(rename = "totalContributions")]
    total_contributions: u32,
    weeks: Vec<Week>,
}

#[derive(Deserialize, Debug)]
struct Week {
    #[serde(rename = "contributionDays")]
    contribution_days: Vec<ContributionDay>,
}

#[derive(Deserialize, Debug)]
struct ContributionDay {
    date: String,
    #[serde(rename = "contributionCount")]
    contribution_count: u32,
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
    maybe_token: Option<String>,
) -> Result<Activity, Box<dyn std::error::Error>> {
    let number_of_weeks = ((date_range.1 - date_range.0).num_days() as f32 / 7.0).ceil() as usize;

    let mut contributions: HashMap<YearWeek, HashMap<Weekday, u32>> = HashMap::new();

    // Initialize the activity weeks
    for date in date_range.0.iter_weeks().take(number_of_weeks) {
        let year_week = get_year_week(date);
        contributions.insert(year_week, HashMap::new());
    }

    let client = reqwest::Client::new();

    // Format dates for GraphQL query (ISO 8601 format)
    let from = format!("{}T00:00:00Z", date_range.0);
    let to = format!("{}T23:59:59Z", date_range.1);

    // GraphQL query to fetch contribution calendar
    let graphql_query = r#"
        query($username: String!, $from: DateTime!, $to: DateTime!) {
            user(login: $username) {
                contributionsCollection(from: $from, to: $to) {
                    contributionCalendar {
                        totalContributions
                        weeks {
                            contributionDays {
                                date
                                contributionCount
                            }
                        }
                    }
                }
            }
        }
    "#;

    let request_body = GraphQLRequest {
        query: graphql_query.to_string(),
        variables: GraphQLVariables {
            username: user.to_string(),
            from,
            to,
        },
    };

    // Build the request
    let mut request = client
        .post("https://api.github.com/graphql")
        .header("User-Agent", "gh-trophy")
        .json(&request_body);

    // Add authentication if token is provided
    if let Some(token) = maybe_token {
        request = request.bearer_auth(token);
    }

    // Make the request
    let response = request.send().await?;

    // Check for errors
    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(format!("GitHub API error {}: {}", status, response_text).into());
    }

    // Parse the GraphQL response
    let graphql_response: GraphQLResponse = serde_json::from_str(&response_text)?;

    // Process the contribution calendar data
    for week in graphql_response
        .data
        .user
        .contributions_collection
        .contribution_calendar
        .weeks
    {
        for day in week.contribution_days {
            if day.contribution_count > 0 {
                // Parse the date
                if let Ok(date) = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d") {
                    if date >= date_range.0 && date <= date_range.1 {
                        let year_week = get_year_week(date);
                        let weekday = date.weekday();

                        let entry = contributions.entry(year_week).or_insert_with(HashMap::new);
                        *entry.entry(weekday).or_insert(0) += day.contribution_count;
                    }
                }
            }
        }
    }

    Ok(Activity {
        date_range,
        contributions,
    })
}
