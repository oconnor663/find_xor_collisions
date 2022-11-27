use anyhow::{anyhow, Result};
use std::array;
use std::cmp;

const LEN: usize = 3;
const N: usize = 3;

fn random_vec() -> [u8; LEN] {
    array::from_fn(|_| rand::random::<u8>() % 2)
}

fn add<const SIZE: usize>(a: [u8; SIZE], b: [u8; SIZE]) -> [u8; SIZE] {
    array::from_fn(|i| (a[i] + b[i]) % 2)
}

#[test]
fn test_add() {
    let a = [0, 1, 0];
    let b = [1, 1, 0];
    let c = [1, 0, 0];
    assert_eq!(add(a, b), c);
}

fn print_row<const SIZE: usize>(vec: [u8; SIZE]) {
    print!("[");
    for x in vec {
        print!("{}", x);
    }
    println!("]");
}

fn print_matrix(matrix: [[u8; N + 1]; LEN]) {
    for row in matrix {
        print_row(row);
    }
}

fn row_eschelon_form(mut matrix: [[u8; N + 1]; LEN]) -> Result<[[u8; N + 1]; LEN]> {
    for col in 0..cmp::min(N, LEN) {
        // Find a row below our current position (which is `col`) with this column set.
        let mut found_row = Err(anyhow!("no row found"));
        for row in col..N {
            if matrix[row][col] == 1 {
                found_row = Ok(row);
                break;
            }
        }
        // Swap the row we found (if any) with the current one (index `col`), or bail if we didn't
        // find one.
        matrix.swap(col, found_row?);
        // Eliminate this column from all rows below.
        for row in col + 1..N {
            if matrix[row][col] == 1 {
                matrix[row] = add(matrix[row], matrix[col]);
            }
        }
    }
    Ok(matrix)
}

fn make_matrix(vecs: [[u8; LEN]; N], target: [u8; LEN]) -> [[u8; N + 1]; LEN] {
    let mut matrix = [[0; N + 1]; LEN];
    for row in 0..LEN {
        for col in 0..N {
            matrix[row][col] = vecs[col][row];
        }
        matrix[row][N] = target[row];
    }
    matrix
}

fn main() {
    const LEN: usize = 3;
    const N: usize = 3;

    let vecs: [[u8; LEN]; N] = array::from_fn(|_| random_vec());
    let target = random_vec();
    println!("vecs:");
    for v in vecs {
        print_row(v);
    }
    println!();
    println!("target:");
    print_row(target);
    println!();
    let matrix = make_matrix(vecs, target);
    print_matrix(matrix);
}
