use scraper::*;

pub async fn fetch_doc(url: &str) -> Result<Html, reqwest::Error> {
    let client = reqwest::Client::builder().build()?;
    let res = client.get(url).send().await?.text().await?;
    Ok(Html::parse_document(&res))
}

pub async fn fetch_restaurants(searchfield: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = [("searchfield", searchfield)];
    let result = client
        .post("https://www.sv-restaurant.ch/de/mensen/mensa-suche?type=8700")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    // println!("{}", result);

    Ok(result)
}
