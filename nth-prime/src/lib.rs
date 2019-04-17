#![feature(test)]
extern crate test;

#[bench]
fn bench_big_prime(b: &mut test::Bencher) {
    b.iter(|| nth(10001));
}

#[bench]
fn bench_big_prime_another_func(b: &mut test::Bencher) {
    b.iter(|| another_nth(10001));
}

// $ rustup toolchain install nightly
// $ cargo +nightly bench
// test bench_big_prime ... bench:  38,757,730 ns/iter (+/- 4,888,749)
// In other words, 0.04 seconds to find 10001st prime
pub fn nth(n: usize) -> Result<usize, ()> {

    if n == 0 {
        return Err(());
    }

    let index = n - 1;
    let mut limit = 2;
    loop {
        let primes = primes_upto(limit);

        if let Some(nth_prime) = primes.get(index) {
            return Ok(*nth_prime);
        } else {
            limit *= 2;
        }
    }
}

/// Find all the primes up to limit
/// Adapted from https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn primes_upto(limit: usize) -> Vec<usize> {
    let n = limit + 1;
    let mut composites = std::collections::HashSet::new();
    let mut primes = vec![2];

    // Enumerate the multiples of p by counting to n from 2p in increments of p,
    // and mark them in the list (these will be 2p, 3p, 4p, ...; the p itself
    // should not be marked).
    'outer: loop {
        let p = *primes.last().unwrap();
        let mut mark = p * p;
        while mark < n {
            composites.insert(mark);
            mark += p;
        }

        // Find the first number greater than p in the list that is not marked.
        // If there was no such number, stop. Otherwise, let p now equal this
        // new number (which is the next prime), and repeat from previous step.
        for candidate in (p + 1)..n {
            if !composites.contains(&candidate) {
                primes.push(candidate);
                continue 'outer;
            }
        }

        break;
    }

    primes
}


pub fn another_nth(n: i32) -> i32 {
    let mut primes = vec![2];
    let mut num: i32 = 3;
    while (primes.len() as i32) <= n {
        let mut stop_loop = false;
        for p in primes.iter_mut() {
            if num % *p == 0 {
                stop_loop = true;
                break;
            }
        }
        if !stop_loop {
            primes.push(num);
        }
        num += 2;
    }
    primes.last().cloned().unwrap()
}


//pub fn another_nth(n: u32) -> u32 {
//    (2u32..).filter(|x| is_prime(x)).nth(n as usize).unwrap()
//}
//
//fn is_prime(n: &u32) -> bool {
//    !(2..n - 1).any(|y| n % y == 0)
//}
