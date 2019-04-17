#![feature(test)]
extern crate test;

#[bench]
fn bench_sum_of_multiples_hash_set_n_10000_k_5(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_hash_set(10000, &[2, 5, 15, 43, 47]));
}

#[bench]
fn bench_sum_of_multiples_nk_n_10000_k_5(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_nk(10000, &[2, 5, 15, 43, 47]));
}

#[bench]
fn bench_sum_of_multiples_2_to_pow_k_n_10000_k_5(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_2_to_pow_k(10000, &[2, 5, 15, 43, 47]));
}

#[bench]
fn bench_sum_of_multiples_itertools_n_10000_k_5(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_itertools(10000, &[2, 5, 15, 43, 47]));
}

#[bench]
fn bench_sum_of_multiples_hash_set_n_10000_k_20(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_hash_set(10000, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
}

#[bench]
fn bench_sum_of_multiples_nk_n_10000_k_20(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_nk(10000, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
}

#[bench]
fn bench_sum_of_multiples_2_to_pow_k_n_10000_k_20(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_2_to_pow_k(10000, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
}

#[bench]
fn bench_sum_of_multiples_itertools_n_10000_k_20(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_itertools(10000, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
}

#[bench]
fn bench_sum_of_multiples_hash_set_n_1000000_k_20(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_hash_set(1000000, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
}

#[bench]
fn bench_sum_of_multiples_nk_n_1000000_k_20(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_nk(1000000, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
}

#[bench]
fn bench_sum_of_multiples_2_to_pow_k_n_1000000_k_20(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_2_to_pow_k(1000000, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
}

#[bench]
fn bench_sum_of_multiples_itertools_n_1000000_k_20(b: &mut test::Bencher) {
    b.iter(|| sum_of_multiples_itertools(1000000, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]));
}

// $ rustup toolchain install nightly
// $ cargo +nightly bench
//test bench_sum_of_multiples_hash_set_n_10000_k_5      ... bench:     474,965 ns/iter (+/- 47,226)
//test bench_sum_of_multiples_nk_n_10000_k_5            ... bench:     211,026 ns/iter (+/- 12,177)
//test bench_sum_of_multiples_2_to_pow_k_n_10000_k_5    ... bench:       2,927 ns/iter (+/- 592)
//test bench_sum_of_multiples_itertools_n_10000_k_5     ... bench:      67,225 ns/iter (+/- 6,621)
//
//test bench_sum_of_multiples_hash_set_n_10000_k_20     ... bench:   1,266,410 ns/iter (+/- 725,451)
//test bench_sum_of_multiples_nk_n_10000_k_20           ... bench:     434,083 ns/iter (+/- 17,497)
//test bench_sum_of_multiples_2_to_pow_k_n_10000_k_20   ... bench: 383,150,137 ns/iter (+/- 64,818,642)
//test bench_sum_of_multiples_itertools_n_10000_k_20    ... bench:     379,054 ns/iter (+/- 136,330)
//
//test bench_sum_of_multiples_hash_set_n_1000000_k_20   ... bench: 325,367,070 ns/iter (+/- 51,569,778)
//test bench_sum_of_multiples_nk_n_1000000_k_20         ... bench:  44,450,276 ns/iter (+/- 3,617,468)
//test bench_sum_of_multiples_2_to_pow_k_n_1000000_k_20 ... bench: 351,476,486 ns/iter (+/- 12,217,726)
//test bench_sum_of_multiples_itertools_n_1000000_k_20  ... bench:  61,320,609 ns/iter (+/- 9,105,487)

use std::collections::HashSet;

pub fn sum_of_multiples_hash_set(limit: u64, factors: &[u64]) -> u64 {
    factors.iter()
           .filter(|&n| *n > 0)
           .map(|z| (*z..limit).step_by(*z as usize))
           .flatten()
           .collect::<HashSet<_>>().iter().sum()
}


pub fn sum_of_multiples_nk(limit: u64, factors: &[u64]) -> u64 {
    (1..limit).filter(|&x| factors.iter().filter(|&n| *n > 0)
              .any(|n| x % n == 0))
              .sum()
}

// credit to Tim Vermeulen
pub fn sum_of_multiples_2_to_pow_k(limit: u64, factors: &[u64]) -> u64 {
    let factors = || factors.iter().cloned().filter(|&f| f > 0);

    (1_u64..1 << factors().count()).fold(0, |total, x| {
        let factors = factors()
            .enumerate()
            .filter(|&(i, _)| (1 << i) & x != 0)
            .map(|(_, f)| f);

        let multiple = factors.fold(1, lcm);
        let count = (limit - 1) / multiple;
        let sum = multiple * count * (count + 1) / 2;

        if x.count_ones() % 2 == 0 {
            total - sum
        } else {
            total + sum
        }
    })
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y > 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

use itertools::Itertools;

pub fn sum_of_multiples_itertools(limit: u64, factors: &[u64]) -> u64 {
    factors.iter()
           .filter(|&n| *n > 0)
           .map(|z| (*z..limit).step_by(*z as usize))
           .flatten().sorted().dedup().sum()
}