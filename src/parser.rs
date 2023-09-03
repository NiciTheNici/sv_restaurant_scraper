use crate::fetch;
use crate::structs::*;
use chrono::prelude::*;
use scraper::*;
use url::Url;

pub fn get_menus(document: &Html) -> Result<Vec<Menu>, Box<dyn std::error::Error>> {
    let weekday_selector = Selector::parse("div.menu-plan-grid")?;
    let mealname_selector = Selector::parse("h2.menu-title")?;

    let selected_mealnames = document.select(&weekday_selector);
    match selected_mealnames.size_hint() {
        (.., Some(_)) => {
            let mut menus = Vec::new();
            let dates = parse_dates(document)?;
            // println!("{:#?}", dates);

            for (i, day) in selected_mealnames.enumerate() {
                let mut meals = Vec::new();
                for meal in day.select(&mealname_selector) {
                    let meal = Meal {
                        name: meal.inner_html(),
                        description: String::from("placeholder"),
                    };
                    meals.push(meal);
                }

                let current_date = dates[i];

                menus.push(Menu {
                    date: current_date,
                    meals,
                });
            }
            Ok(menus)
        }
        (.., None) => Err(ScrapeError {
            message: String::from("No meals found"),
        }
        .into()),
    }
}

pub async fn get_restaurants(
    document: &Html,
    base_url: Url,
) -> Result<Vec<Restaurant>, Box<dyn std::error::Error>> {
    let restaurant_nav_selector = Selector::parse("div.restaurant-nav")?;
    let restaurant_name_selector = Selector::parse("a")?;

    let restaurant_nav = document.select(&restaurant_nav_selector).next();
    let mut restaurants: Vec<Restaurant> = Vec::new();

    match restaurant_nav {
        Some(restaurant_nav) => {
            for restaurant in restaurant_nav.select(&restaurant_name_selector) {
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
        }
        None => {
            let current_restaurant_selector = Selector::parse("div.name-wrap")?;
            let current_restaurant_name_selector = Selector::parse("a")?;
            let current_restaurant = document
                .select(&current_restaurant_selector)
                .next()
                .unwrap()
                .select(&current_restaurant_name_selector)
                .next()
                .unwrap()
                .inner_html();
            restaurants.push(Restaurant {
                name: current_restaurant,
                link: base_url.to_string(),
                menus: Vec::new(),
            })
        }
    }

    for restaurant in &mut restaurants {
        let doc = fetch::fetch_doc(&restaurant.link).await?;
        restaurant.menus.append(&mut get_menus(&doc)?);
    }
    Ok(restaurants)
}
fn parse_dates(document: &Html) -> Result<Vec<NaiveDate>, Box<dyn std::error::Error>> {
    let date_selector = Selector::parse("span.date")?;
    let scraped_dates = document.select(&date_selector);
    let mut dates = Vec::new();

    for scraped_date in scraped_dates {
        let mut scraped_date_string = scraped_date.inner_html();
        scraped_date_string.push_str(Local::now().year().to_string().as_str());
        dates.push(NaiveDate::parse_from_str(
            scraped_date_string.as_str(),
            "%d.%m.%Y",
        )?);
    }

    Ok(dates)
}
