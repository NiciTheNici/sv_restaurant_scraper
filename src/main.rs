mod structs;
use crate::structs::*;
use colored::Colorize;
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
        for meal in day.select(&meal_selector) {
            let meal = Meal {
                name: meal.inner_html(),
                description: String::from("placeholder"),
            };
            println!("{}", meal.name);
        }
    }

    Ok(())
}

async fn get_svrestaurant_html(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::builder().build()?;
    client.get(url).send().await?.text().await
}
