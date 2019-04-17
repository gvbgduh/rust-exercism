pub fn factors(n: u64) -> Vec<u64> {
    let mut current = n;
    let mut primes = vec![];
    while current != 1 {
        for i in 2..=current {
            if current % i == 0 {
                primes.push(i);
                current /= i;
                break;
            }
        }
    }
    primes
}
