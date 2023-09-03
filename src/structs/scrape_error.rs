use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct ScrapeError {
    pub message: String,
}

impl std::error::Error for ScrapeError {}

impl Display for ScrapeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "No menus found in provided sv-restaurant URL")
    }
}
