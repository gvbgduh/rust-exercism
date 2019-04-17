#![feature(test)]
extern crate test;

#[bench]
fn bench_sq_shift(b: &mut test::Bencher) {
    b.iter(|| total_shift());
}

#[bench]
fn bench_sq(b: &mut test::Bencher) {
    b.iter(|| total());
}

// $ rustup toolchain install nightly
// $ cargo +nightly bench
//test bench_sq       ... bench:           0 ns/iter (+/- 0)
//test bench_sq_shift ... bench:         258 ns/iter (+/- 11)

pub fn square_shift(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2_u64.pow(s - 1)
}

pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    1 << (s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}

pub fn total_shift() -> u64 {
    (1..=64).map(square_shift).sum()
}