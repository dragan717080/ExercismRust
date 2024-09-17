fn main() {
    let input = 10;
    let output = primes_up_to(input);
    let expected = [2, 3, 5, 7];
    assert_eq!(output, expected);
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }

    let mut prime = vec![true; upper_bound as usize + 1];

    let mut p: usize = 2;

    while p.pow(2) <= upper_bound as usize {
        // Update all p multiples
        if prime[p] == true {
            for i in (p.pow(2)..upper_bound as usize + 1).step_by(p) {
                prime[i] = false;
            }
        }

        p += 1;
    }

    (2..upper_bound + 1).filter_map(|d| if prime[d as usize] { Some(d) } else { None }).collect()
}
