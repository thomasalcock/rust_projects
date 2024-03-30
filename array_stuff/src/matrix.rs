use ::rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub struct Matrix {
    pub nrows: usize,
    pub ncols: usize,
    pub data: Vec<f32>,
}

impl Matrix {
    pub fn random(nrows: usize, ncols: usize, min: f32, max: f32) -> Matrix {
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

    pub fn print_matrix(&self) {
        println!("\nnrows: {}, ncols: {}", self.nrows, self.ncols);
        for i in 0..self.nrows {
            let start: usize = i * self.ncols;
            println!("{:?}", &self.data[start..(start + self.ncols)]);
        }
    }

    pub fn dimensions(&self) {
        println!("nrows={}, ncols={}", self.nrows, self.ncols);
    }

    pub fn multiply(&self, m: &Matrix) -> Matrix {
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

    pub fn append_column(&mut self, m: &Matrix) {
        if m.ncols != 1 {
            panic!("m must have exactly one column");
        }
        let mut index: usize;     
        for i in 0..m.nrows {
            index = (i * self.ncols) + self.ncols;
            if i > 0 {
                index += 1;
            }
            self.data.insert(index, m.data[i]);
        };
        // first insert data, then resize columns because
        // otherwise awkward helper variable (previous ncols) or -1 on index
        // is required
        self.ncols += 1;
    }

/*

have:
[1.0 2.0]
[3.0 4.0]

after inserting the column[5,6] I want:
[1.0, 2.0, 5.0]
[3.0, 4.0, 6.0]

have:
[1.0, 2.0, 5.0] 2 +ncols i = 0 -> 
[3.0, 6.0, 4.0] 5 +ncols
[7.0  8.0  9.0] 8 +ncols
[7.0  8.0  9.0] 11 +ncols
*/
    // TODO: this could be ddone in placee, mey be more efficient?
    pub fn add(&self, m: &Matrix) -> Matrix {
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
