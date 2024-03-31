#![cfg(test)]

use rand::random;

use crate::matrix::*;

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
    assert_eq!(c.ncols, expected.ncols);
    assert_eq!(c.nrows, expected.nrows);
}

#[test]
fn append_inplace_column_works() {
    let mut a: Matrix = Matrix {
        ncols: 2,
        nrows: 2,
        data: vec![1.0, 2., 3., 4.],
    };
    let expected: Matrix = Matrix {
        ncols: 3,
        nrows: 2,
        data: vec![1., 2., 5., 3., 4., 6.],
    };
    let column: Matrix = Matrix{ncols: 1, nrows: 2, data: vec![5., 6.]};
    a.append_column(&column);
    expected.print_matrix();
    a.print_matrix();
    assert_eq!(a.data, expected.data);
    assert_eq!(a.nrows, expected.nrows);
    assert_eq!(a.ncols, expected.ncols);
}

#[test]
fn append_inplace_column_works_2() {
    let mut a: Matrix = Matrix::random(10, 2, 0.0, 1.0);
    let column: Matrix = Matrix{ncols: 1, nrows: a.nrows, data: vec![1.0; a.nrows]};
    a.append_column(&column);
    a.print_matrix();
    assert_eq!(a.nrows, 10);
    assert_eq!(a.ncols, 3);
}
