use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Error {
    pub(crate) message: String,
}

impl Error {
    pub(crate) fn new(message: String) -> Self {
        Error { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}
