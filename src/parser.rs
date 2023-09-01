use crate::fetch;
use crate::structs::*;
use chrono::prelude::*;
use scraper::*;
use url::Url;

pub fn get_menus(document: &Html) -> Result<Vec<Menu>, Box<dyn std::error::Error>> {
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

pub async fn get_restaurants(
    document: &Html,
    base_url: Url,
) -> Result<Vec<Restaurant>, Box<dyn std::error::Error>> {
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
            link: format!(
                "https://{}{}",
                base_url.host_str().unwrap(),
                restaurant.value().attr("href").unwrap()
            ),
            menus: Vec::new(),
        })
    }
    for restaurant in &mut restaurants {
        let doc = fetch::fetch_doc(&restaurant.link).await?;
        restaurant.menus.append(&mut get_menus(&doc)?);
    }
    Ok(restaurants)
}
fn parse_date(mut scraped_date: String) -> Result<NaiveDate, chrono::ParseError> {
    scraped_date.push_str(Local::now().year().to_string().as_str());
    // println!("{}", scraped_date);
    NaiveDate::parse_from_str(scraped_date.as_str(), "%d.%m.%Y")
}
