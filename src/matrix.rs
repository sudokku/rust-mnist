use rand::prelude::*;
use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
pub struct Matrix{
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}

impl Matrix{
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix{
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows]
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
            
        }

        res
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Matrices are of incorrect sizes")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        res
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Matrices are of incorrect sizes")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }

        res
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("Matrices are of incorrect sizes")
        }

        let mut res =  Matrix::zeros(self.rows, rhs.cols);
        
        for i in 0..res.rows {
            for j in 0..res.cols {
                let mut sum = 0.0;
                
                for k in 0..self.cols {
                    sum += self.data[i][k] * rhs.data[k][j];
                }

                res.data[i][j] = sum;
            }
        }

        res
    }
}
