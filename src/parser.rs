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
        (.., None) => Err(SvError {
            message: String::from("No meals found"),
        }
        .into()),
    }
}

pub fn get_base_url(document: &Html) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url_selector = Selector::parse(r#"meta[property="og:url"]"#)?;
    let base_url = match document.select(&base_url_selector).next() {
        Some(res) => Ok(res.value().attr("content")),
        None => Err(SvError {
            message: String::from("Could not get base url"),
        }),
    };
    match base_url? {
        Some(res) => Ok(Url::parse(res)?),
        None => Err(SvError {
            message: String::from("Url property does not exist"),
        }
        .into()),
    }
}

pub async fn get_restaurants(
    document: &Html,
) -> Result<Vec<Restaurant>, Box<dyn std::error::Error>> {
    let base_url = get_base_url(document)?;
    let restaurant_nav_selector = Selector::parse("div.restaurant-nav")?;
    let restaurant_name_selector = Selector::parse("a")?;

    let restaurant_nav = document.select(&restaurant_nav_selector).next();
    let mut restaurants: Vec<Restaurant> = Vec::new();

    match restaurant_nav {
        Some(restaurant_nav) => {
            for restaurant in restaurant_nav.select(&restaurant_name_selector) {
                let host_str = match base_url.host_str() {
                    Some(host_str) => Ok(host_str),
                    None => Err(SvError {
                        message: String::from("Error parsing host_str"),
                    }),
                };
                let link_subdirectory = match restaurant.value().attr("href") {
                    Some(link_subdirectory) => Ok(link_subdirectory),
                    None => Err(SvError {
                        message: String::from("Failed to get subdirectory from restaurant"),
                    }),
                };

                let link = format!("https://{}{}", host_str?, link_subdirectory?);
                restaurants.push(Restaurant {
                    name: restaurant.inner_html(),
                    link,
                    menus: Vec::new(),
                })
            }
        }
        None => {
            let restaurant_selector = Selector::parse("div.name-wrap")?;
            let restaurant_name_selector = Selector::parse("a")?;
            let restaurant_fragment = match document.select(&restaurant_selector).next() {
                Some(res) => Ok(res),
                None => Err(SvError {
                    message: String::from("Could not find div named \"name-wrap\""),
                }),
            };
            let restaurant = match restaurant_fragment?
                .select(&restaurant_name_selector)
                .next()
            {
                Some(res) => Ok(res),
                None => Err(SvError {
                    message: String::from("Could not find \"anchor\" inside div \"name-wrap\""),
                }),
            };
            restaurants.push(Restaurant {
                name: restaurant?.inner_html(),
                link: base_url.to_string(),
                menus: Vec::new(),
            })
        }
    }

    for restaurant in &mut restaurants {
        let url = Url::parse(restaurant.link.as_str())?;
        let doc = fetch::fetch_doc(&url).await?;
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
