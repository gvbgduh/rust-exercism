#![feature(test)]
extern crate test;

use std::cmp;
use std::collections::HashSet;
use rayon::iter::{IntoParallelIterator, ParallelIterator};


#[bench]
fn bench_find_old_30000000(b: &mut test::Bencher) {
    b.iter(|| find_old(30000000));
}

#[bench]
fn bench_find_one_30000000(b: &mut test::Bencher) {
    b.iter(|| find_one(30000000));
}

#[bench]
fn bench_find_30000000(b: &mut test::Bencher) {
    b.iter(|| find(30000000));
}

#[bench]
fn bench_find_filter_map_30000000(b: &mut test::Bencher) {
    b.iter(|| find_filter_map(30000000));
}

#[bench]
fn bench_find_filter_map_advanced_30000000(b: &mut test::Bencher) {
    b.iter(|| find_filter_map_advanced(30000000));
}

#[bench]
fn bench_find_in_one_filter_map_rayon_30000000(b: &mut test::Bencher) {
    b.iter(|| find_in_one_filter_map_rayon(30000000));
}

#[bench]
fn bench_find_in_one_filter_map_30000000(b: &mut test::Bencher) {
    b.iter(|| find_in_one_filter_map(30000000));
}

// $ rustup toolchain install nightly
// $ cargo +nightly bench

// Initial run
// ---------------------------------------------------------------------
// test bench_find_30000     ... bench:      45,922 ns/iter (+/- 4,353)
// test bench_find_old_30000 ... bench:       2,729 ns/iter (+/- 512)
// test bench_find_one_30000 ... bench:      45,135 ns/iter (+/- 3,944)

// Reduced from sum/2 to sum/3
// ---------------------------------------------------------------------
// test bench_find_30000     ... bench:      32,314 ns/iter (+/- 1,711)
// test bench_find_old_30000 ... bench:       2,593 ns/iter (+/- 426)
// test bench_find_one_30000 ... bench:      31,719 ns/iter (+/- 6,439)

// Using the rayon's par_iter
// ---------------------------------------------------------------------
// test bench_find_30000     ... bench:     207,379 ns/iter (+/- 15,673)
// test bench_find_old_30000 ... bench:       2,705 ns/iter (+/- 503)
// test bench_find_one_30000 ... bench:      29,052 ns/iter (+/- 3,945)

// Using the rayon's par_iter + `filter_map`
// ---------------------------------------------------------------------
// test bench_find_30000            ... bench:     208,807 ns/iter (+/- 17,645)
// test bench_find_filter_map_30000 ... bench:     199,157 ns/iter (+/- 7,745)
// test bench_find_old_30000        ... bench:       2,734 ns/iter (+/- 499)
// test bench_find_one_30000        ... bench:      29,197 ns/iter (+/- 1,410)

// Using only one filter_map
// ---------------------------------------------------------------------
//test bench_find_30000000                         ... bench:  49,888,851 ns/iter (+/- 5,246,306)
//test bench_find_filter_map_30000000              ... bench:  48,908,808 ns/iter (+/- 20,001,854)
//test bench_find_filter_map_advanced_30000000     ... bench:  40,205,861 ns/iter (+/- 10,419,169)
//test bench_find_in_one_filter_map_30000000       ... bench:  27,062,113 ns/iter (+/- 1,461,191)
//test bench_find_in_one_filter_map_rayon_30000000 ... bench:  39,306,770 ns/iter (+/- 6,089,863)
//test bench_find_old_30000000                     ... bench:      55,102 ns/iter (+/- 3,758)
//test bench_find_one_30000000                     ... bench:  27,499,324 ns/iter (+/- 3,292,044)

def gen_num():
    a = 1
    while True:
        yield a
        a += 1

def gen_gen(g):
    yield from g

fn gcd(mut x: u32, mut y: u32) -> u32 {
    while y > 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn grab_solutions(sum: u32, m: u32) -> HashSet<[u32; 3]> {
    let mut k: u32;
    let mut n: u32;
    let mut d: u32;
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut solutions = HashSet::new();

    k = match m % 2 {
        0 => m + 1,
        _ => m + 2,
    };

    while k < 2 * m && k <= sum / (2 * m) {
        if sum / (2 * m) % k == 0 && gcd(k, m) == 1 {
            d = sum / 2 / (k * m);
            n = k - m;
            a = d * (m.pow(2) - n.pow(2));
            b = 2 * d * n * m;
            c = d * (m.pow(2) + n.pow(2));
            let final_a = cmp::min(a, b);
            let final_b = cmp::max(a, b);
            solutions.insert([final_a, final_b, c]);
        }
        k += 2;
    }
    solutions
}

pub fn find_old(sum: u32) -> HashSet<[u32; 3]> {
    let limit = ((sum/2) as f32).sqrt() as u32;
    (2..=limit).filter(|m| sum % m == 0 && (sum / 2) % m == 0)
               .map(|m| grab_solutions(sum, m))
               .flatten()
               .collect::<HashSet<_>>()
}

pub fn find_one(sum: u32) -> HashSet<[u32; 3]> {
    let mut solutions = HashSet::new();

    for side_a in 1..=sum/3 {
        let b_plus_c: u32 = sum - side_a;
        let side_b = (b_plus_c.pow(2) - side_a.pow(2)) / (2 * b_plus_c);
        let side_c = b_plus_c - side_b;
        if side_a < side_b && side_a.pow(2) + side_b.pow(2) == side_c.pow(2) {
            solutions.insert([side_a, side_b, side_c]);
        }
    }
    solutions
}

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1_u32..sum/3).into_par_iter()
               .map(|a| (a, sum - a))
               .map(|(a, b_and_c)| (a, b_and_c, (b_and_c.pow(2) - a.pow(2)) / (2 * b_and_c)))
               .map(|(a, b_and_c, b)| [a, b, b_and_c - b])
               .filter(|[a, b, c]| a < b && a.pow(2) + b.pow(2) == c.pow(2))
               .collect::<HashSet<_>>()
}

pub fn find_filter_map(sum: u32) -> HashSet<[u32; 3]> {
    (1_u32..sum/3).into_par_iter()
               .map(|a| (a, sum - a))
               .map(|(a, b_and_c)| (a, b_and_c, (b_and_c.pow(2) - a.pow(2)) / (2 * b_and_c)))
               .filter_map(
                   |(a, b_and_c, b)| if a < b && a.pow(2) + b.pow(2) == (b_and_c - b).pow(2) {Some([a, b, b_and_c - b])} else { None }
                )
               .collect::<HashSet<_>>()
}

pub fn find_filter_map_advanced(sum: u32) -> HashSet<[u32; 3]> {
    (1_u32..(sum / 3_u32)).into_par_iter()
        .filter_map(|side_a| {
            let b_plus_c: u32 = sum - side_a;
            let side_b: u32 = (b_plus_c.pow(2) - side_a.pow(2)) / (b_plus_c * 2);
            let side_c: u32 = b_plus_c - side_b;

            let is_a_less_than_b: bool = side_a < side_b;
            let are_sides_valid_pythagorean: bool = side_a.pow(2) + side_b.pow(2) == side_c.pow(2);
            let are_both_conditions_met: bool = is_a_less_than_b && are_sides_valid_pythagorean;

            match are_both_conditions_met {
                true => {
                    Some([side_a, side_b, side_c])
                },
                false => None,
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}

pub fn find_in_one_filter_map_rayon(sum: u32) -> HashSet<[u32; 3]> {
    (1_u32..(sum / 3_u32)).into_par_iter()
        .filter_map(|side_a| {
            let b_plus_c: u32 = sum - side_a;
            let side_b: u32 = (b_plus_c.pow(2) - side_a.pow(2)) / (b_plus_c * 2);
            let side_c: u32 = b_plus_c - side_b;

            let is_a_less_than_b: bool = side_a < side_b;
            let are_sides_valid_pythagorean: bool = side_a.pow(2) + side_b.pow(2) == side_c.pow(2);
            let are_both_conditions_met: bool = is_a_less_than_b && are_sides_valid_pythagorean;

            match are_both_conditions_met {
                true => {
                    Some([side_a, side_b, side_c])
                },
                false => None,
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}

pub fn find_in_one_filter_map(sum: u32) -> HashSet<[u32; 3]> {
    (1_u32..(sum / 3_u32))
        .filter_map(|side_a| {
            let b_plus_c: u32 = sum - side_a;
            let side_b: u32 = (b_plus_c.pow(2) - side_a.pow(2)) / (b_plus_c * 2);
            let side_c: u32 = b_plus_c - side_b;

            let is_a_less_than_b: bool = side_a < side_b;
            let are_sides_valid_pythagorean: bool = side_a.pow(2) + side_b.pow(2) == side_c.pow(2);
            let are_both_conditions_met: bool = is_a_less_than_b && are_sides_valid_pythagorean;

            match are_both_conditions_met {
                true => {
                    Some([side_a, side_b, side_c])
                },
                false => None,
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}
