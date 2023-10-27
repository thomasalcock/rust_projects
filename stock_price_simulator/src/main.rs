extern crate csv;
extern crate ndarray;
extern crate ndarray_rand;

use csv::Writer;
use std::error::Error;

use ndarray::{Array, Array2};
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::RandomExt;

fn main() -> Result<(), Box<dyn Error>> {
    let output_path: &str = "outputs/output.csv";

    // Set parameters
    let n_paths = 50;
    let n_steps = 1000;
    let maturity = 1.0;
    let r = 0.05;
    let sigma = 0.4;
    let s_0 = 100.0;

    // Generate random numbers
    let z_values = Array::random((n_steps, n_paths), Normal::new(r, sigma).unwrap());

    // Perform calculations
    let dt = maturity / n_steps as f64;
    let middle_term = (r - 0.5 * sigma * sigma) * dt;
    let sqrt_dt = sigma * f64::sqrt(dt);

    // Vector to store paths
    let mut paths: Array2<f64> = Array2::<f64>::zeros((n_steps + 1, n_paths));
    paths.fill(f64::ln(s_0));

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
    let mut writer = Writer::from_path(output_path)?;
    writer.write_record(&["path_index", "time_index", "price"])?;

    for i in 0..n_steps {
        for j in 0..n_paths {
            let j_str: String = j.to_string();
            let ti_str = time_index[i].to_string();
            let p_str = paths[[i, j]].to_string();
            let _ = writer.write_record(&[j_str, ti_str, p_str]);
        }
    }

    println!("Wrote file to {}", output_path);
    Ok(())
}
