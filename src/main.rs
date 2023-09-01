mod fetch;
mod parser;
mod structs;
use crate::structs::*;
use colored::Colorize;
use std::fs;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_restaurant_url = Url::parse(&fs::read_to_string("./restaurant.txt")?)?;
    let document = fetch::fetch_doc(base_restaurant_url.as_str()).await?;

    let restaurants = parser::get_restaurants(&document, base_restaurant_url);
    let menus = parser::get_menus(&document)?;

    println!("{:#?}", restaurants.await?);
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
