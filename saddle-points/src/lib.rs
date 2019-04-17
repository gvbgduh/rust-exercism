#![feature(test)]
extern crate test;

#[bench]
fn test_first_iter(b: &mut test::Bencher) {
    b.iter(|| find_saddle_points_first(&vec![vec![5, 5, 5, 5, 5, 5, 5], vec![5, 5, 5, 5, 5, 5, 5], vec![5, 5, 5, 5, 5, 5, 5]]));
}

#[bench]
fn test_second_iter(b: &mut test::Bencher) {
    b.iter(|| find_saddle_points(&vec![vec![5, 5, 5, 5, 5, 5, 5], vec![5, 5, 5, 5, 5, 5, 5], vec![5, 5, 5, 5, 5, 5, 5]]));
}

// $ rustup toolchain install nightly
// $ cargo +nightly bench
// ====================================================================
// test test_first_iter  ... bench:       1,161 ns/iter (+/- 151)
// test test_second_iter ... bench:       1,288 ns/iter (+/- 252)


pub fn find_saddle_points_first(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let empty_vec = input.iter().any(|x| x.is_empty());
    
    if empty_vec {
        return Vec::new()
    }

    let rows_max = input.iter().map(|row| row.iter().max().unwrap()).collect::<Vec<_>>();
    let mut result = vec![];

    if rows_max.is_empty() {
        return Vec::new()
    }

    for (idx_row, row) in input.iter().enumerate() {
        for (idx_col, col) in row.iter().enumerate() {
            if col == rows_max[idx_row] {
                let min_in_col = input.iter().map(|a| a[idx_col]).min().unwrap();
                if *rows_max[idx_row] == min_in_col {
                    result.push((idx_row, idx_col));
                }
            }
        }
    }
    result
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let empty_vec = input.iter().any(|x| x.is_empty());

    if empty_vec {
        return Vec::new()
    }

    let cols_len = input[0].len();

    let rows_max = input.iter().map(|row| row.iter().max().unwrap()).collect::<Vec<_>>();
    let cols_min = (0..cols_len).map(|col_idx| input.iter().map(|a| a[col_idx]).min().unwrap()).collect::<Vec<_>>();
    let mut result = vec![];

    for (idx_row, row) in input.iter().enumerate() {
        for (idx_col, col) in row.iter().enumerate() {
            if col == rows_max[idx_row] && col == &cols_min[idx_col] {
                result.push((idx_row, idx_col));
            }
        }
    }
    result
}