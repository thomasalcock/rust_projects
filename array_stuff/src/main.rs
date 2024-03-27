use ::rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug)]
struct Matrix {
    nrows: usize,
    ncols: usize,
    data: Vec<f32>,
}

impl Matrix {
    fn random(nrows: usize, ncols: usize, min: f32, max: f32) -> Matrix {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut matrix: Matrix = Matrix {
            nrows: nrows,
            ncols: ncols,
            data: vec![0.0; nrows * ncols],
        };
        for i in 0..(nrows * ncols) {
            matrix.data[i] = rng.gen_range(min..max);
        }
        return matrix;
    }

    fn print_matrix(&self) {
        println!("\nnrows: {}, ncols: {}", self.nrows, self.ncols);
        for i in 0..self.nrows {
            let start: usize = i * self.ncols;
            println!("{:?}", &self.data[start..(start + self.ncols)]);
        }
    }

    fn dimensions(&self) {
        println!("nrows={}, ncols={}", self.nrows, self.ncols);
    }

    fn multiply(&self, m: &Matrix) -> Matrix {
        if self.ncols != m.nrows {
            panic!("k != n");
        }
        let mut result: Matrix = Matrix {
            nrows: self.nrows,
            ncols: m.ncols,
            data: vec![0.0; self.nrows * m.ncols],
        };
        let mut a: f32;
        let mut b: f32;

        for i in 0..self.nrows {
            for j in 0..m.ncols {
                for k in 0..self.ncols {
                    a = self.data[i * self.ncols + k];
                    b = m.data[k * m.ncols + j];
                    result.data[i * result.ncols + j] += a * b;
                }
            }
        }

        return result;
    }

    fn add(&self, m: &Matrix) -> Matrix {
        let mut result: Matrix = Matrix {
            nrows: self.nrows,
            ncols: m.ncols,
            data: vec![0.0; self.nrows * self.ncols],
        };
        let mut lhs: f32;
        let mut rhs: f32;
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                rhs =  m.data[i * self.ncols + j];
                lhs = self.data[i * self.ncols + j];
                result.data[i * self.ncols + j] = lhs + rhs;
            }
        }
        return result;
    }
}

fn main() {
    let features: Matrix = Matrix::random(10, 2, 10.0, 40.0);
    let outputs: Matrix = Matrix::random(10, 1, 5.0, 6.0);
    let mut weights: Matrix = Matrix::random(2, 1, 0.1, 1.0);
    
    let alpha: f32 = 0.01;
    let n_epochs: usize = 5;

    for _  in 0..n_epochs {
        let predictions: Matrix = features.multiply(&weights);
        let mut error: f32 = 0.0;
        let n_obs: usize  = outputs.data.len();

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_multiplication_works() {
        let a: Matrix = Matrix {
            ncols: 2,
            nrows: 2,
            data: vec![1.0, 2., 3., 4.],
        };
        let b: Matrix = Matrix {
            ncols: 2,
            nrows: 2,
            data: vec![5., 6., 7., 8.],
        };
        let c: Matrix = a.multiply(&b);
        let expected: Matrix = Matrix {
            ncols: 2,
            nrows: 2,
            data: vec![19.0, 22.0, 43.0, 50.0],
        };

        assert_eq!(c.data, expected.data);
        assert_eq!(c.nrows, expected.nrows);
        assert_eq!(c.ncols, expected.ncols);
    }
    #[test]
    fn matrix_add_works() {
        let a: Matrix = Matrix {
            ncols: 2,
            nrows: 2,
            data: vec![1.0, 2., 3., 4.],
        };
        let b: Matrix = Matrix {
            ncols: 2,
            nrows: 2,
            data: vec![5., 6., 7., 8.],
        };
        let c: Matrix = a.add(&b);
        let expected: Matrix = Matrix {
            ncols: 2,
            nrows: 2,
            data: vec![6.0, 8.0, 10.0, 12.0],
        };
        assert_eq!(c.data, expected.data);

    }
}
