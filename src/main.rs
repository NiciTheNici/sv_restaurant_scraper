mod structs;
use crate::structs::*;
use colored::Colorize;
use datetime::{convenience::Today, LocalDate, Month};
use reqwest::Error;
use scraper::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res =
        get_svrestaurant_html("https://giardino-sg.sv-restaurant.ch/de/menuplan/giardino/").await?;
    let document = Html::parse_document(&res);
    let day_selector = Selector::parse("div.menu-plan-grid").unwrap();
    let meal_selector = Selector::parse("h2.menu-title").unwrap();
    let date_selector = Selector::parse("span.date").unwrap();
    // let title = document.select(&title_selector).next().unwrap();

    for (i, day) in document.select(&day_selector).enumerate() {
        println!(
            "{}",
            document
                .select(&date_selector)
                .nth(i)
                .unwrap()
                .inner_html()
                .red()
        );
        let mut meals = Vec::new();
        for meal in day.select(&meal_selector) {
            let meal = Meal {
                name: meal.inner_html(),
                description: String::from("placeholder"),
            };
            println!("{}", &meal.name);
            meals.push(meal);
        }
        // let date_html = day.select(&date_selector).next().unwrap().inner_html();
        // let mut day_number = date_html.split(".");
        // println!("{}", day_number.nth(0).unwrap());
        let menu = Menu {
            date: LocalDate::ymd(2023, Month::from_one(06)?, 16)?,
            meals,
        };
    }

    Ok(())
}

async fn get_svrestaurant_html(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::builder().build()?;
    client.get(url).send().await?.text().await
}
