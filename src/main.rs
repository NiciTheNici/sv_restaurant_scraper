mod fetch;
mod parser;
mod structs;
use crate::structs::*;
use std::fs;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // print_restaurants().await
    println!("{:#?}", fetch::fetch_restaurants("bkw").await?);

    Ok(())
}

async fn print_restaurants() -> Result<(), Box<dyn std::error::Error>> {
    let base_restaurant_url = Url::parse(&fs::read_to_string("./restaurant.txt")?)?;
    let document = fetch::fetch_doc(base_restaurant_url.as_str()).await?;

    let restaurants = parser::get_restaurants(&document, base_restaurant_url).await?;

    for restaurant in restaurants {
        println!("{}", restaurant);
    }
    Ok(())
}
