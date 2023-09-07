use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RestaurantSearchResponse {
    result: Result,
    list: Vec<SearchedRestaurant>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SearchedRestaurant {
    address: String,
    id: String,
    lat: String,
    lng: String,
    link: String,
    name: String,
    #[serde(rename = "type")]
    publicity: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Result {
    amount: u32,
}
