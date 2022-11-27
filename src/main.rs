use std::array;
use std::cmp;

const LEN: usize = 256;
const N: usize = 256;

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

fn row_eschelon_form(mut matrix: [[u8; N + 1]; LEN]) -> [[u8; N + 1]; LEN] {
    let mut current_row = 0;
    for col in 0..cmp::min(N, LEN) {
        // Find a row at or below the current one with this column set.
        let mut found_row = None;
        for row in current_row..N {
            if matrix[row][col] == 1 {
                found_row = Some(row);
                break;
            }
        }
        // If we did find a row, swap it with the current one. If we didn't, keep the current row
        // and continue to the next column.
        if let Some(found_row) = found_row {
            matrix.swap(current_row, found_row);
        } else {
            continue;
        }
        // Eliminate this column from all rows below.
        for row in current_row + 1..N {
            if matrix[row][col] == 1 {
                matrix[row] = add(matrix[row], matrix[current_row]);
            }
        }
        current_row += 1;
    }
    matrix
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

fn back_propagate(mut matrix: [[u8; N + 1]; LEN]) -> [[u8; N + 1]; LEN] {
    for this_row in (0..LEN).rev() {
        let mut first_nonzero_col = None;
        for col in 0..N {
            if matrix[this_row][col] == 1 {
                first_nonzero_col = Some(col);
                break;
            }
        }
        if let Some(col) = first_nonzero_col {
            for other_row in 0..this_row {
                if matrix[other_row][col] == 1 {
                    matrix[other_row] = add(matrix[other_row], matrix[this_row]);
                }
            }
        }
    }
    matrix
}

fn main() {
    let vecs: [[u8; LEN]; N] = array::from_fn(|_| random_vec());
    let target = random_vec();

    // let vecs = [[0, 0, 0], [1, 1, 1], [0, 1, 0]];
    // let target = [0, 1, 1];

    println!("vecs:");
    for v in vecs {
        print_row(v);
    }
    println!();
    println!("target:");
    print_row(target);
    println!();
    println!("transposed equations:");
    let matrix = make_matrix(vecs, target);
    print_matrix(matrix);
    println!();
    println!("row eschelon form:");
    let re_form = row_eschelon_form(matrix);
    print_matrix(re_form);
    println!();
    let reduced_re_form = back_propagate(re_form);
    println!("back propagated:");
    print_matrix(reduced_re_form);

    println!();
    let mut bad = [0; N + 1];
    bad[N] = 1;
    for row in 0..LEN {
        if reduced_re_form[row] == bad {
            println!("NO SOLUTION!");
            return;
        }
    }

    // Collect the results.
    let mut results = [None; N];
    for row in 0..LEN {
        for col in 0..N {
            if reduced_re_form[row][col] == 1 {
                assert!(results[col].is_none());
                results[col] = Some(reduced_re_form[row][N]);
                break;
            }
        }
    }

    // Print the results.
    println!();
    println!("solution:");
    for i in 0..N {
        if let Some(1) = results[i] {
            print_row(vecs[i]);
        }
    }
    for _ in 0..(LEN + 2) {
        print!("-");
    }
    println!();
    print_row(target);
}
