
use reqwest::{self, Response};
use std::fs::File;
use std::io::Write;

pub fn write_currency_price_to_stream(mut stream: &File, currency: &serde_json::Value) {
    let content = format!(
        "{},{},{}\n",
        currency["data"]["symbol"], currency["data"]["priceUsd"], currency["timestamp"]
    );
    println!("{}", content);

    match stream.write_all(content.as_bytes()) {
        Ok(_value) => (),
        Err(err) => println!("Error writing to stream: {}", err),
    };
}

#[tokio::main]
pub async fn get_coin_data(currency: &str) -> serde_json::Value {
    let response: Response =
        match reqwest::get("https://api.coincap.io/v2/assets/".to_string() + currency).await {
            Ok(r) => r,
            Err(err) => {
                panic!("Error getting data from REST API: {}", err)
            }
        };

    let mut snapshot: serde_json::Value = serde_json::Value::Null;

    if response.status().is_success() {
        let body: String = match response.text().await {
            Ok(string) => string,
            Err(err) => {
                panic!("Error extracting text: {}", err)
            }
        };

        snapshot = match serde_json::from_str(&body) {
            Ok(value) => value,
            Err(err) => panic!("Error converting to serde_json::Value: {}", err),
        };
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    snapshot
}
