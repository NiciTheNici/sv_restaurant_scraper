mod fetch;
mod parser;
mod structs;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = fs::read_to_string("./restaurant.txt")?;
    let restaurants = fetch::fetch_restaurants(&url).await?;

    for restaurant in restaurants {
        println!("{}", restaurant);
    }
    Ok(())
}
