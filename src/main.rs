mod parser;
mod structs;
use crate::structs::*;
use colored::Colorize;
use scraper::*;
use std::fs;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let restaurant_url = Url::parse(&fs::read_to_string("./restaurant.txt")?)?;
    let res = get_svrestaurant_html(restaurant_url.as_str()).await?;
    let document = Html::parse_document(&res);

    let restaurants = parser::get_restaurants(&document);
    let menus = parser::get_menus(&document)?;

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

async fn get_svrestaurant_html(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder().build()?;
    client.get(url).send().await?.text().await
}
