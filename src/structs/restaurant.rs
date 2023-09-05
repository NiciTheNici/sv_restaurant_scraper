use chrono::Datelike;
use colored::Colorize;
use inflector::cases::titlecase::to_title_case;

use crate::structs::Menu;
use std::fmt;

#[derive(Debug)]
pub struct Restaurant {
    pub name: String,
    pub link: String,
    pub menus: Vec<Menu>,
}

impl fmt::Display for Restaurant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_string = format!(
            "{}",
            to_title_case(&self.name.to_lowercase().as_str())
                .blue()
                .bold()
        );
        let mut menu_string = String::new();
        for menu in &self.menus {
            menu_string.push_str(
                format!(
                    "{}\n",
                    format!(
                        "{} - {}",
                        menu.date.weekday(),
                        menu.date.format("%d.%m.%Y").to_string()
                    )
                    .red()
                )
                .as_str(),
            );
            for meal in &menu.meals {
                menu_string.push_str(format!("{}\n", meal.name).as_str());
            }
        }
        write!(f, "{}\n{}", name_string, menu_string)
    }
}
