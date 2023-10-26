extern crate csv;
extern crate rand;

use csv::Writer;
use rand_distr::{Distribution, Normal};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let output_path: &str = "outputs/output.csv";

    // Set parameters
    let n_paths = 25;
    let n_steps = 500;
    let maturity = 1.0;
    let r = 0.05;
    let sigma = 0.4;
    let s_0 = 100.0;

    // Generate random numbers
    let normal = Normal::new(r, sigma).unwrap();
    let z_values: Vec<f64> = (0..n_steps * n_paths)
        .map(|_| normal.sample(&mut rand::thread_rng()))
        .collect();

    // Perform calculations
    let dt = maturity / n_steps as f64;
    let middle_term = (r - 0.5 * sigma * sigma) * dt;
    let sqrt_dt = sigma * f64::sqrt(dt);

    // Vector to store paths
    let mut paths: Vec<Vec<f64>> = vec![vec![f64::ln(s_0); n_paths]; n_steps + 1];
    let time_index: Vec<_> = (0..=n_steps).map(|i| i as f64 * dt).collect();

    // Simulate paths
    for i in 0..n_steps {
        for j in 0..n_paths {
            paths[i + 1][j] = paths[i][j] + middle_term + sqrt_dt * z_values[i * n_paths + j];
        }
    }
    for i in 0..n_steps {
        for j in 0..n_paths {
            paths[i][j] = f64::exp(paths[i][j]);
        }
    }

    // Write results to CSV file
    let mut writer = Writer::from_path(output_path)?;
    writer.write_record(&["path_index", "time_index", "price"])?;

    for i in 0..n_steps {
        for j in 0..n_paths {
            let j_str: String = j.to_string();
            let ti_str = time_index[i].to_string();
            let p_str = paths[i][j].to_string();
            let _ = writer.write_record(&[j_str, ti_str, p_str]);
        }
    }

    println!("Simulation results written to output.csv");

    // Calculate summary statistics
    let mut mean_closing_price: f64 = 0.0;
    let mut max_closing_price: f64 = 0.0;

    for j in 0..n_paths {
        let current_price = paths[n_steps][j];
        mean_closing_price += current_price;

        if current_price > max_closing_price {
            max_closing_price = current_price;
        }
    }
    mean_closing_price = mean_closing_price / n_steps as f64;

    println!("mean closing price: {}", mean_closing_price);
    println!("max closing price: {}", max_closing_price);

    Ok(())
}
