fn main() {
    let output = nth(10000);
    let expected = 104743;
    assert_eq!(output, expected);
}

pub fn nth(n: u32) -> u32 {
    let mut count: i32 = -1;
    let mut k = 2;
    let mut last_discovered_prime = 0;

    while count < n as i32 {
        if is_prime(k) {
            count += 1;
            last_discovered_prime = k;
        }

        k += 1;
    }

    last_discovered_prime
}

fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }

    let upper_bound = (n as f64).sqrt() as u32 + 1;

    for k in 2..upper_bound + 1 {
        if n % k == 0 {
            return false;
        }
    }

    true
}
