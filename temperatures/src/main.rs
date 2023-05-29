use std::io;

fn read_line_from_stdin(mut input: String) -> String {
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input
}

fn main() {
    const FIVE_OVER_NINE: f32 = 0.5556;
    const NINE_OVER_FIVE: f32 = 1_f32 / FIVE_OVER_NINE;
    const THIRTY_TWO: f32 = 32.0;

    println!("Convert temperature between Fahrenheit and Degrees.");

    loop {
        println!("Choose unit of input: Fahrenheit (1) or Celsius (2)");
        let unit_type = String::new();
        let temperature = String::new();

        let unit_type = read_line_from_stdin(unit_type);

        let unit_type: u32 = match unit_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                break;
            }
        };

        println!("Enter temperature:");
        let temperature = read_line_from_stdin(temperature);

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                break;
            }
        };

        if unit_type == 1 {
            let celsius: f32 = (temperature - THIRTY_TWO) * FIVE_OVER_NINE;
            println!("{celsius}C = {temperature}F");
        } else if unit_type == 2 {
            let fahrenheit: f32 = NINE_OVER_FIVE * temperature + THIRTY_TWO;
            println!("{fahrenheit}F = {temperature}C");
        } else {
            println!("Please enter 1 (Fahrenheit to Celsius) or 2 (Celius to Fahrenheit)");
            break;
        }
    }
    

}
