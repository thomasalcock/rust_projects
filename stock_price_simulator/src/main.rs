mod cli;
mod file_io;
mod simulation;

use clap::Parser;
use std::error::Error;

use ndarray::{Array, Array2, ArrayBase, Dim, OwnedRepr};
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::RandomExt;

use cli::SimulationArgs;
use file_io::write_simulation_results;
use simulation::simulate_paths;

fn main() -> Result<(), Box<dyn Error>> {
    let args = SimulationArgs::parse();
    let n_steps = args.n_steps;
    let n_paths = args.n_paths;
    let maturity = args.maturity;
    let r = args.r;
    let sigma = args.sigma;
    let initial_stock_price = args.initial_stock_price;
    let output_path = args.output_path;

    let z_values: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> =
        Array::random((n_steps, n_paths), Normal::new(r, sigma).unwrap());

    let mut paths: Array2<f64> = Array2::<f64>::zeros((n_steps + 1, n_paths));
    paths.fill(f64::ln(initial_stock_price));

    let time_index: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> =
        Array::range(0., n_steps as f64, 1.);

    let results: (
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>,
    ) = simulate_paths(
        paths, z_values, time_index, n_paths, n_steps, maturity, r, sigma,
    );

    write_simulation_results(&output_path, results.0, results.1, n_steps, n_paths).unwrap();
    println!("Done.");
    Ok(())
}
