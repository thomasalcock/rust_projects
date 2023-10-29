extern crate clap;
extern crate csv;
extern crate ndarray;
extern crate ndarray_rand;

use csv::Writer;
use std::error::Error;
use std::usize;

use ndarray::{Array, Array2};
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::RandomExt;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SimulationArgs {
    #[arg(long, default_value_t = 25)]
    n_paths: usize,

    #[arg(long, default_value_t = 100)]
    n_steps: usize,

    #[arg(long, default_value_t = 1.0)]
    maturity: f64,

    #[arg(long, default_value_t = 0.04)]
    r: f64,

    #[arg(long, default_value_t = 0.5)]
    sigma: f64,

    #[arg(long, default_value_t = 100.0)]
    initial_stock_price: f64,

    #[arg(long)]
    output_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get and check command line args
    let args = SimulationArgs::parse();
    let n_steps = args.n_steps;
    let n_paths = args.n_paths;
    let maturity = args.maturity;
    let r = args.r;
    let sigma = args.sigma;
    let initial_stock_price = args.initial_stock_price;
    let output_path = args.output_path;

    // Generate random numbers
    let z_values = Array::random((n_steps, n_paths), Normal::new(r, sigma).unwrap());

    // Perform calculations
    let dt = maturity / n_steps as f64;
    let middle_term = (r - 0.5 * sigma * sigma) * dt;
    let sqrt_dt = sigma * f64::sqrt(dt);

    // Vector to store paths
    let mut paths: Array2<f64> = Array2::<f64>::zeros((n_steps + 1, n_paths));
    paths.fill(f64::ln(initial_stock_price));

    let mut time_index = Array::range(0., n_steps as f64, 1.);
    time_index = time_index.map(|i| i as &f64 * dt);

    // Simulate paths
    for i in 0..n_steps {
        for j in 0..n_paths {
            paths[[i + 1, j]] = paths[[i, j]] + middle_term + sqrt_dt * z_values[[i, j]];
        }
    }
    for i in 0..n_steps {
        for j in 0..n_paths {
            paths[[i, j]] = f64::exp(paths[[i, j]]);
        }
    }

    // Write results to CSV file
    println!("Writing file to {}", output_path);
    let mut writer = Writer::from_path(output_path)?;
    writer.write_record(&["path_index", "time_index", "price"])?;

    for i in 0..n_steps {
        for j in 0..n_paths {
            let _ = writer.write_record(&[
                j.to_string(),
                time_index[i].to_string(),
                paths[[i, j]].to_string(),
            ]);
        }
    }
    println!("Done.");
    Ok(())
}
