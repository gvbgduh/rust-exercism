#![feature(test)]
extern crate test;


#[bench]
fn bench_one(b: &mut test::Bencher) {
    b.iter(|| series_one("1234567890", 2));
}

#[bench]
fn bench_two(b: &mut test::Bencher) {
    b.iter(|| series_two("1234567890", 2));
}

#[bench]
fn bench_three(b: &mut test::Bencher) {
    b.iter(|| series_three("1234567890", 2));
}

#[bench]
fn bench_four(b: &mut test::Bencher) {
    b.iter(|| series_four("1234567890", 2));
}

// $ rustup toolchain install nightly
// $ cargo +nightly bench
// test bench_four  ... bench:       1,794 ns/iter (+/- 163)
// test bench_one   ... bench:       1,571 ns/iter (+/- 177)
// test bench_three ... bench:       3,280 ns/iter (+/- 348)
// test bench_two   ... bench:       1,185 ns/iter (+/- 162)


pub fn series_one(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_owned(); digits.len() + 1]
    }
    digits.chars()
          .collect::<Vec<_>>()
          .windows(len)
          .map(|a| a.into_iter()
          .collect::<String>()).collect::<Vec<_>>()
}

pub fn series_two(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {return vec![] }
    (0..digits.len()-len+1).map(|i| {
        digits[i..i+len].to_string()
    }).collect()
}

pub fn series_three(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_owned(); 6];
    }
    digits
        .chars()
        .map(|x| format!("{}", x))
        .collect::<Vec<_>>()
        .windows(len)
        .map(|x| x.concat())
        .collect()
}

pub fn series_four(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();

    if len > digits.len() {
        return result;
    }

    for start in 0..(digits.len() - len + 1) {
        result.push( String::from(&digits[start..start+len]) );
    }

    result
}
