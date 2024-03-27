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
        for _ in 0..(nrows * ncols) {
            matrix.data.push(rng.gen_range(min..max));
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
}

fn main() {
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

    a.print_matrix();
    b.print_matrix();
    c.print_matrix();
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
}
