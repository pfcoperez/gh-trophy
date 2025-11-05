use chrono::NaiveDate;
use std::collections::HashMap;

type UserName<'a> = &'a str;

type DateRange = (NaiveDate, NaiveDate);

enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub struct Activity {
    date_range: DateRange,
    contributions: Vec<HashMap<DayOfWeek, u32>>,
}

pub async fn get_activity(_user: UserName<'_>) -> Activity {
    // Implementation to fetch and return user activity
    unimplemented!()
}
