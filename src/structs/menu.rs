use crate::Meal;
use datetime::LocalDate;

pub struct Menu {
    date: LocalDate,
    meals: Vec<Meal>,
}
