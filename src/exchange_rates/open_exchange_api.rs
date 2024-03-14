use reqwest;

use crate::ExchangeRate;

const API_KEY: &str = "55aa1e90398e410fbbac6b94c270cbef";

#[tokio::main]
pub async fn get_current_exchangerates() -> ExchangeRate {
    let url = format!(
        "https://openexchangerates.org/api/latest.json?\
    app_id={api_key}",
        api_key = API_KEY
    );

    let json_response = reqwest::get(&url).await.unwrap().text().await.unwrap();

    serde_json::from_str(&json_response).unwrap()
}
