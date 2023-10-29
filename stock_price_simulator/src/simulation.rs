use ndarray::{Array, Array2, ArrayBase, Dim, OwnedRepr};
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::RandomExt;

pub struct SimulationInputs {
    pub random_values: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    pub paths: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    pub time_index: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>,
}

pub fn init_simulation_inputs(
    n_steps: usize,
    n_paths: usize,
    r: f64,
    sigma: f64,
    initial_stock_price: f64,
) -> SimulationInputs {
    let time_index: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> =
        Array::range(0., n_steps as f64, 1.);

    let z_values: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> =
        Array::random((n_steps, n_paths), Normal::new(r, sigma).unwrap());

    let mut paths: Array2<f64> = Array2::<f64>::zeros((n_steps + 1, n_paths));
    paths.fill(f64::ln(initial_stock_price));

    SimulationInputs {
        random_values: z_values,
        paths: paths,
        time_index: time_index,
    }
}

pub fn simulate_paths(
    mut simulation_inputs: SimulationInputs,
    n_paths: usize,
    n_steps: usize,
    maturity: f64,
    r: f64,
    sigma: f64,
) -> SimulationInputs {
    let dt = maturity / n_steps as f64;
    let middle_term = (r - 0.5 * sigma * sigma) * dt;
    let sqrt_dt = sigma * f64::sqrt(dt);
    simulation_inputs.time_index = simulation_inputs.time_index.map(|i| i as &f64 * dt);

    for i in 0..n_steps {
        for j in 0..n_paths {
            simulation_inputs.paths[[i + 1, j]] = simulation_inputs.paths[[i, j]]
                + middle_term
                + sqrt_dt * simulation_inputs.random_values[[i, j]];
        }
    }
    for i in 0..n_steps {
        for j in 0..n_paths {
            simulation_inputs.paths[[i, j]] = f64::exp(simulation_inputs.paths[[i, j]]);
        }
    }

    return simulation_inputs;
}
