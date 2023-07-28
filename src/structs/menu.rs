use crate::Meal;
use datetime::LocalDate;

pub struct Menu {
    pub date: LocalDate,
    pub meals: Vec<Meal>,
}
