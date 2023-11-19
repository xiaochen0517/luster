use chrono::NaiveDate;
use regex::Regex;

use crate::models::error::Error;

pub(crate) fn due_date_to_naive_date(due_date: &str) -> Result<NaiveDate, Error> {
    // check if due_date is in the format YYYY-MM-DD
    let regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").map_err(|e| Error::new(e.to_string()))?;
    if !regex.is_match(due_date) {
        return Err(Error::new(format!(
            "Invalid due date format: {}. Expected format: YYYY-MM-DD",
            due_date
        )));
    }
    Ok(NaiveDate::parse_from_str(due_date, "%Y-%m-%d").map_err(|e| Error::new(e.to_string()))?)
}
