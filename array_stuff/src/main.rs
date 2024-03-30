// TODO: test column insert function
// TODO: add constant to feature matrix
// TODO: finish estimation of weights
// TODO: refactor and check which operations can be inplace


mod matrix;
mod tests;

use matrix::Matrix;

fn main() {
    let mut features: Matrix = Matrix::random(10, 2, 10.0, 40.0);
    let outputs: Matrix = Matrix::random(10, 1, 5.0, 6.0);
    let mut weights: Matrix = Matrix::random(2, 1, 0.1, 1.0);

    let alpha: f32 = 0.01;
    let n_epochs: usize = 5;

    for _ in 0..n_epochs {
        let predictions: Matrix = features.multiply(&weights);
        let mut error: f32 = 0.0;
        let n_obs: usize = outputs.data.len();

        for i in 0..n_obs {
            error += (predictions.data[i] - outputs.data[i]).powf(2.0) / n_obs as f32;
        }
        for i in 0..weights.nrows {
            weights.data[i] -= alpha * error;
        }
        println!("error = {}", error);
        weights.print_matrix();
    }
}
