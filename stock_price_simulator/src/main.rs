mod cli;
mod file_io;
mod simulation;

use clap::Parser;
use std::error::Error;

use cli::SimulationArgs;
use file_io::write_simulation_results;
use simulation::{init_simulation_inputs, simulate_paths, SimulationInputs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: SimulationArgs = SimulationArgs::parse();
    let n_steps: usize = args.n_steps;
    let n_paths: usize = args.n_paths;
    let maturity: f64 = args.maturity;
    let r: f64 = args.r;
    let sigma: f64 = args.sigma;
    let initial_stock_price: f64 = args.initial_stock_price;
    let output_path: String = args.output_path;

    let sim_inputs: SimulationInputs =
        init_simulation_inputs(n_steps, n_paths, r, sigma, initial_stock_price);

    let sim_inputs: SimulationInputs =
        simulate_paths(sim_inputs, n_paths, n_steps, maturity, r, sigma);

    write_simulation_results(&output_path, sim_inputs, n_steps, n_paths).unwrap();
    println!("Done.");
    Ok(())
}
