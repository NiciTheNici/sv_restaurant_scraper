mod fetch;
mod parser;
mod structs;
use crate::structs::*;
use colored::Colorize;
use inflector::cases::titlecase::to_title_case;
use std::fs;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_restaurant_url = Url::parse(&fs::read_to_string("./restaurant.txt")?)?;
    let document = fetch::fetch_doc(base_restaurant_url.as_str()).await?;

    let restaurants = parser::get_restaurants(&document, base_restaurant_url).await?;

    pretty_print_restaurants(restaurants);
    Ok(())
}

fn pretty_print_restaurants(restaurants: Vec<Restaurant>) {
    for restaurant in restaurants {
        println!();
        println!(
            "{}",
            to_title_case(restaurant.name.to_lowercase().as_str())
                .blue()
                .bold()
        );
        for menu in restaurant.menus {
            println!("{}", menu.date.format("%d.%m").to_string().red());
            for meal in menu.meals {
                println!("{}", meal.name);
            }
        }
    }
}
