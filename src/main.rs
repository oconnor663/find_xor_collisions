use rand::prelude::*;
use std::array;

const WORDS: &str = include_str!("../words.txt");
const NUM_WORDS: usize = 936;

const LEN: usize = 256;
const N: usize = NUM_WORDS;

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

fn row_eschelon_form(matrix: &mut [[u8; N + 1]; LEN]) {
    let mut current_row = 0;
    for col in 0..N {
        // Find a row at or below the current one with this column set.
        let mut found_row = None;
        for row in current_row..LEN {
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
        for row in (current_row + 1)..LEN {
            if matrix[row][col] == 1 {
                matrix[row] = add(matrix[row], matrix[current_row]);
            }
        }
        current_row += 1;
        // Quit if we've run out of rows.
        if current_row == LEN {
            break;
        }
    }
}

fn make_matrix(vecs: &[[u8; LEN]; N], target: &[u8; LEN]) -> Box<[[u8; N + 1]; LEN]> {
    let mut matrix = Box::new([[0; N + 1]; LEN]);
    for row in 0..LEN {
        for col in 0..N {
            matrix[row][col] = vecs[col][row];
        }
        matrix[row][N] = target[row];
    }
    matrix
}

fn back_propagate(matrix: &mut [[u8; N + 1]; LEN]) {
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
}

fn vec_from_str(s: &str) -> [u8; LEN] {
    let hash = blake3::hash(s.as_bytes());
    let mut v = [0; LEN];
    for byte_index in 0..hash.as_bytes().len() {
        for bit_index in 0..8 {
            v[8 * byte_index + bit_index] = (hash.as_bytes()[byte_index] >> bit_index) & 1;
        }
    }
    v
}

fn find_solution(target_str: &str, words: &[&'static str]) -> Vec<&'static str> {
    // The words may be shuffled, but they should still all be there.
    assert_eq!(words.len(), NUM_WORDS);

    let vecs = Box::new(array::from_fn(|i| vec_from_str(&words[i])));
    let target = vec_from_str(target_str);

    let mut matrix = make_matrix(&vecs, &target);

    row_eschelon_form(&mut matrix);

    back_propagate(&mut matrix);

    let mut bad = [0; N + 1];
    bad[N] = 1;
    for row in 0..LEN {
        if matrix[row] == bad {
            eprintln!("NO SOLUTION!");
            std::process::exit(1);
        }
    }

    // Collect the results.
    let mut result_words = Vec::new();
    for row in 0..LEN {
        for col in 0..N {
            if matrix[row][col] == 1 {
                if matrix[row][N] == 1 {
                    result_words.push(words[col]);
                }
                break;
            }
        }
    }

    // Assert that the results are correct.
    let target_hash = *blake3::hash(target_str.as_bytes()).as_bytes();
    let mut sum = [0u8; 32];
    for &word in &result_words {
        for (i, b) in blake3::hash(word.as_bytes())
            .as_bytes()
            .into_iter()
            .enumerate()
        {
            sum[i] ^= b;
        }
    }
    assert_eq!(sum, target_hash);

    result_words
}

fn print_json_words(words: &[&str]) {
    // Print the results as JSON.
    print!("[");
    let mut first = true;
    for word in words {
        if first {
            first = false;
        } else {
            print!(", ");
        }
        print!("{:?}", word);
    }
    println!("]");
}

fn main() {
    let target_str = "hello world";
    println!("for target string: {:?}", target_str);
    let mut words: Vec<&str> = WORDS.split_whitespace().collect();

    // Find the shortest result we can.
    let mut best = usize::MAX;
    let mut rng = rand::thread_rng();
    loop {
        words.shuffle(&mut rng);
        let result_words = find_solution(target_str, &words);
        if result_words.len() < best {
            println!("found a solution of length {}", result_words.len());
            best = result_words.len();
            print_json_words(&result_words);
        }
    }
}
