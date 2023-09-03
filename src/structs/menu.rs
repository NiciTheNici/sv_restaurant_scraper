use crate::Meal;
use chrono::prelude::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Menu {
    pub date: NaiveDate,
    pub meals: Vec<Meal>,
}

#[derive(Debug)]
pub struct MenuError;

impl std::error::Error for MenuError {}

impl Display for MenuError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "No menus found in provided sv-restaurant URL")
    }
}
