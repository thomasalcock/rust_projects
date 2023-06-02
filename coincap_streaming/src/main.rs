mod coincapapi;
mod utils;

use coincapapi::{get_coin_data, write_currency_price_to_stream};
use utils::create_or_append_output_file;

use std::thread;
use std::time::Duration;

fn main() {
    println!("--- CoinCap Data Stream v0.1.0 ---");

    let stream_file: &str = "stream.txt";
    let output_stream = create_or_append_output_file(stream_file);

    loop {
        let currency_data = get_coin_data("bitcoin");
        write_currency_price_to_stream(&output_stream, &currency_data);

        let currency_data = get_coin_data("ethereum");
        write_currency_price_to_stream(&output_stream, &currency_data);

        thread::sleep(Duration::from_secs(60)); // API updates every minute
    }
}
