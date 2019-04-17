#![feature(test)]
extern crate test;



#[bench]
fn test_collatz_naive(b: &mut test::Bencher) {
    b.iter(|| collatz_naive(test::black_box(10000000000000)));
}

#[bench]
fn test_collatz_recursive(b: &mut test::Bencher) {
    b.iter(|| collatz_recursive(test::black_box(10000000000000)));
}

#[bench]
fn test_collatz_rec_without_map(b: &mut test::Bencher) {
    b.iter(|| collatz_rec_without_map(test::black_box(10000000000000)));
}

// $ rustup toolchain install nightly
// $ cargo +nightly bench
// NOTE: black_box is used to aviod the compiler optimisation and have the honest benchmarking
// =========================================================================
// test test_collatz_naive           ... bench:         348 ns/iter (+/- 71)
// test test_collatz_rec_without_map ... bench:       1,067 ns/iter (+/- 97)
// test test_collatz_recursive       ... bench:         620 ns/iter (+/- 113)


pub fn collatz_naive(n: u64) -> Option<u64> {
    if n == 0 {
        return None
    }
    
    let mut counter = 0;
    let mut val = n;

    while val != 1 {
        match val % 2 {
            0 => val /= 2 as u64,
            1 => val = 3 * val + 1,
            _ => panic!("?!")
        }
        counter += 1;
    }
    Some(counter)
}

pub fn collatz_recursive(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz_recursive(n / 2).map(|x| x + 1),
        n => collatz_recursive(3 * n + 1).map(|x| x + 1)
    }
}

pub fn collatz_rec_without_map(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ if n % 2 == 0 => Some(collatz_rec_without_map(n/2)?+1),
        _ => Some(collatz_rec_without_map(n*3+1)?+1),
    }
}