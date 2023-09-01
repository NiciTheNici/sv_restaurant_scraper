use scraper::*;

pub async fn fetch_doc(url: &str) -> Result<Html, reqwest::Error> {
    let client = reqwest::Client::builder().build()?;
    let res = client.get(url).send().await?.text().await?;
    Ok(Html::parse_document(&res))
}
