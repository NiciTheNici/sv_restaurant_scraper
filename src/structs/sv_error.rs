use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct SvError {
    pub message: String,
}

impl std::error::Error for SvError {}

impl Display for SvError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "No menus found in provided sv-restaurant URL")
    }
}
