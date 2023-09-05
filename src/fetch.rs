use crate::parser::get_restaurants;
use scraper::*;
use url::Url;

use crate::structs::Restaurant;

pub async fn fetch_doc(url: &Url) -> Result<Html, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let url = Url::parse(url.as_str())?;
    let res = client.get(url).send().await?.text().await?;
    Ok(Html::parse_document(&res))
}

pub async fn fetch_restaurants(
    url: &String,
) -> Result<Vec<Restaurant>, Box<dyn std::error::Error>> {
    let url = Url::parse(url.as_str())?;
    let document = fetch_doc(&url).await?;
    get_restaurants(&document, url).await
}
