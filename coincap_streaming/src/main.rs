use reqwest::{self, Response};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    let stream_file: &str = "stream.txt";
    let path = Path::new(&stream_file);
    let file_exists: bool = path.exists() && path.is_file();
    let output_stream: File = create_output_stream(stream_file);
    if file_exists {
        println!(
            "The file {} already exists. Data will be appended to this file.",
            stream_file
        );
    } else {
        println!("The file does not yet exist!");
    }

    loop {
        let currency_data = get_coin_data("bitcoin");
        write_currency_price_to_stream(&output_stream, &currency_data);

        let currency_data = get_coin_data("ethereum");
        write_currency_price_to_stream(&output_stream, &currency_data);

        thread::sleep(Duration::from_secs(60)); // API updates every minute
    }
}

fn write_currency_price_to_stream(mut stream: &File, currency: &serde_json::Value) {
    let content = format!(
        "{},{}",
        currency["data"]["symbol"], currency["data"]["priceUsd"],
    );

    match stream.write_all(content.as_bytes()) {
        Ok(_value) => (),
        Err(err) => println!("Error writing to stream: {}", err),
    };
}

#[tokio::main]
async fn get_coin_data(currency: &str) -> serde_json::Value {
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

fn create_output_stream(file_path: &str) -> std::fs::File {
    let file = match OpenOptions::new().create(true).append(true).open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening the file: {}", err),
    };

    file
}
