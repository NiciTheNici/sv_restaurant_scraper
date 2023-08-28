use crate::Meal;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Menu {
    pub date: NaiveDate,
    pub meals: Vec<Meal>,
}
