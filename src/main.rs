use reqwest::Error;
use scraper::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res =
        get_svrestaurant_html("https://giardino-sg.sv-restaurant.ch/de/menuplan/giardino/").await?;
    let document = Html::parse_document(&res);
    let title_selector = Selector::parse("h2").unwrap();
    let title = document.select(&title_selector).next().unwrap();
    println!("{}", title.inner_html());
    Ok(())
}

async fn get_svrestaurant_html(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::builder().build()?;
    client.get(url).send().await?.text().await
}
