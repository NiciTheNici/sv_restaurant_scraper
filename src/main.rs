mod structs;
use crate::structs::*;
use chrono::prelude::*;
use colored::Colorize;
use scraper::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res =
        get_svrestaurant_html("https://giardino-sg.sv-restaurant.ch/de/menuplan/giardino/").await?;
    let document = Html::parse_document(&res);
    let day_selector = Selector::parse("div.menu-plan-grid").unwrap();
    let meal_selector = Selector::parse("h2.menu-title").unwrap();
    let date_selector = Selector::parse("span.date").unwrap();
    let mut menus = Vec::new();

    for (i, day) in document.select(&day_selector).enumerate() {
        let date = parse_date(document.select(&date_selector).nth(i).unwrap().inner_html())?;
        // println!("date: {}", date.to_string().red());

        let mut meals = Vec::new();
        for meal in day.select(&meal_selector) {
            let meal = Meal {
                name: meal.inner_html(),
                description: String::from("placeholder"),
            };
            // println!("{}", &meal.name);
            meals.push(meal);
        }
        menus.push(Menu { date, meals });
    }
    pretty_print_menu(menus);
    Ok(())
}

fn pretty_print_menu(menus: Vec<Menu>) {
    for menu in menus {
        println!("{}", menu.date.format("%d.%m").to_string().red());
        for meal in menu.meals {
            println!("{}", meal.name);
        }
    }
}

fn parse_date(mut scraped_date: String) -> Result<NaiveDate, chrono::ParseError> {
    scraped_date.push_str(Local::now().year().to_string().as_str());
    // println!("{}", scraped_date);
    NaiveDate::parse_from_str(scraped_date.as_str(), "%d.%m.%Y")
}

async fn get_svrestaurant_html(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder().build()?;
    client.get(url).send().await?.text().await
}
