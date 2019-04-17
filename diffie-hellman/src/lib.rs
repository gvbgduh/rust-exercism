use rand::seq::SliceRandom;

pub fn private_key(p: u64) -> u64 {
    let mut primes = vec![2, 3];
    
    for el in (5..p).step_by(2) {
        if !primes.iter().any(|&x| el % x == 0) {
            primes.push(el);
        }
    }

    *primes.choose(&mut rand::thread_rng()).unwrap()
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow_low_mem(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow_low_mem(b_pub, a, p)
}

fn modular_pow_low_mem(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0
    }

    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }

        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }

    result
}
