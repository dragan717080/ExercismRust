fn main() {
    let n = 93819012551;

    println!("{:?}", factors(n));

    let n = 8;

    println!("{:?}", factors(n));
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;

    let mut i = 2;

    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1;
    }

    // If n is greater than 1, it is a prime factor
    if n > 1 {
        factors.push(n);
    }

    factors
}
