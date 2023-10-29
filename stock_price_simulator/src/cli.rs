use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SimulationArgs {
    #[arg(long, default_value_t = 25)]
    pub n_paths: usize,

    #[arg(long, default_value_t = 100)]
    pub n_steps: usize,

    #[arg(long, default_value_t = 1.0)]
    pub maturity: f64,

    #[arg(long, default_value_t = 0.04)]
    pub r: f64,

    #[arg(long, default_value_t = 0.5)]
    pub sigma: f64,

    #[arg(long, default_value_t = 100.0)]
    pub initial_stock_price: f64,

    #[arg(long)]
    pub output_path: String,
}
