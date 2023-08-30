mod structs;
use crate::structs::*;
use chrono::prelude::*;
use colored::Colorize;
use scraper::{error::SelectorErrorKind, *};
use std::fs;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let restaurant_url = Url::parse(&fs::read_to_string("./restaurant.txt")?)?;
    let res = get_svrestaurant_html(restaurant_url.as_str()).await?;
    let document = Html::parse_document(&res);

    let restaurants = get_restaurants(&document);
    let menus = get_menus(&document)?;

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

fn get_menus(document: &Html) -> Result<Vec<Menu>, Box<dyn std::error::Error>> {
    //TODO replace this box with something better
    let weekday_selector = Selector::parse("div.menu-plan-grid")?;
    let date_selector = Selector::parse("span.date")?;
    let mealname_selector = Selector::parse("h2.menu-title")?;

    let mut menus = Vec::new();

    for (i, day) in document.select(&weekday_selector).enumerate() {
        let date = parse_date(document.select(&date_selector).nth(i).unwrap().inner_html())?;

        let mut meals = Vec::new();
        for meal in day.select(&mealname_selector) {
            let meal = Meal {
                name: meal.inner_html(),
                description: String::from("placeholder"),
            };
            meals.push(meal);
        }
        menus.push(Menu { date, meals });
    }

    Ok(menus)
}

fn get_restaurants(document: &Html) -> Result<Vec<Restaurant>, SelectorErrorKind> {
    let restaurant_nav_selector = Selector::parse("div.restaurant-nav")?;
    let restaurant_name_selector = Selector::parse("a")?;

    let mut restaurants: Vec<Restaurant> = Vec::new();

    for restaurant in document
        .select(&restaurant_nav_selector)
        .next()
        .unwrap()
        .select(&restaurant_name_selector)
    {
        restaurants.push(Restaurant {
            name: restaurant.inner_html(),
            link: format!("{}", restaurant.value().attr("href").unwrap()),
            menus: Vec::new(),
        })
    }
    Ok(restaurants)
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
