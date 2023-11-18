use chrono::NaiveDate;

use crate::models::error::Error;

pub(crate) fn due_date_to_naive_date(due_date: &str) -> Result<NaiveDate, Error> {
    Ok(NaiveDate::parse_from_str(due_date, "%Y-%m-%d").map_err(|e| Error::new(e.to_string()))?)
}
