use chrono::{NaiveDate, Duration};

pub fn generate_date_range(start: &str, end: &str) -> Vec<String> {
    let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap();

    let mut date_range = Vec::new();
    let mut current_date = start_date;

    while current_date <= end_date {
        date_range.push(current_date.to_string());
        current_date = current_date + Duration::days(1);
    }

    date_range
}