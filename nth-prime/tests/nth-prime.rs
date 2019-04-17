use nth_prime as np;
use std::time::Instant;


#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    let now = Instant::now();
    let res = np::nth(13000);
    println!("Time: {} s.", now.elapsed().as_secs());
    assert_eq!(res, 104743);
}

#[test]
fn test_same_big_prime_another_func() {
    let now = Instant::now();
    let res = np::another_nth(13000);
    println!("Time: {} s.", now.elapsed().as_secs());
    assert_eq!(res, 104743);
}