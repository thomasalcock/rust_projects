use ndarray::{ArrayBase, Dim, OwnedRepr};

pub fn simulate_paths(
    mut simulation_data: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    z_values: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    mut time_index: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>,
    n_paths: usize,
    n_steps: usize,
    maturity: f64,
    r: f64,
    sigma: f64,
) -> (
    ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>,
) {
    let dt = maturity / n_steps as f64;
    let middle_term = (r - 0.5 * sigma * sigma) * dt;
    let sqrt_dt = sigma * f64::sqrt(dt);
    time_index = time_index.map(|i| i as &f64 * dt);

    for i in 0..n_steps {
        for j in 0..n_paths {
            simulation_data[[i + 1, j]] =
                simulation_data[[i, j]] + middle_term + sqrt_dt * z_values[[i, j]];
        }
    }
    for i in 0..n_steps {
        for j in 0..n_paths {
            simulation_data[[i, j]] = f64::exp(simulation_data[[i, j]]);
        }
    }

    return (simulation_data, time_index);
}
