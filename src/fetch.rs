use scraper::*;

pub async fn fetch_doc(url: &str) -> Result<Html, reqwest::Error> {
    let client = reqwest::Client::builder().build()?;
    let res = client.get(url).send().await?.text().await?;
    Ok(Html::parse_document(&res))
}

pub async fn fetch_restaurants(searchfield: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = [("searchfield", searchfield)];
    // let result = client
    //     .post("https://www.sv-restaurant.ch/de/mensen/mensa-suche?type=8700")
    //     .form(&params)
    //     .send()
    //     .await?
    //     .text()
    //     .await?;
    let result = r#"{\"result\":{\"amount\":2,\"sort\":\"name\",\"headerString\":\"{x} Resultate gefunden\"},\"view\":{\"startAmount\":10,\"moreAmount\":10,\"minMoreAmount\":2,\"moreString\":\"N\\u00e4chste {x} anzeigen\",\"start\":{\"lat\":46.84515999999999991132426657713949680328369140625,\"lng\":8.234249999999999403144101961515843868255615234375,\"zoom\":8}},\"list\":[{\"id\":\"res-0224\",\"name\":\"BKW Restaurant atrium\",\"address\":\"Viktoriaplatz 2<br \\\/>\\n3013 Bern\",\"type\":\"\\u00f6ffentlich\",\"distance\":\"\",\"distanceRender\":\"\",\"lat\":\"46.955528\",\"lng\":\"7.451037\",\"link\":\"http:\\\/\\\/bkw-bern.sv-restaurant.ch\",\"linkLabel\":\"Details anzeigen\",\"rendered\":false},{\"id\":\"res-0228\",\"name\":\"BKW Restaurant casamo\",\"address\":\"Galgenfeldweg 18<br \\\/>\\n3016 Bern\",\"type\":\"\\u00f6ffentlich\",\"distance\":\"\",\"distanceRender\":\"\",\"lat\":\"46.954412\",\"lng\":\"7.486185\",\"link\":\"http:\\\/\\\/bkw-ostermundigen.sv-restaurant.ch\",\"linkLabel\":\"Details anzeigen\",\"rendered\":false}]}"#.to_string();

    Ok(result)
}
