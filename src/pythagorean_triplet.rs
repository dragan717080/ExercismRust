use std::collections::HashSet;

fn main() {
    println!("{:?}", find(30000));
}

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    let a_max = sum / 3;

    for a in 1..a_max {
        let b = sum / 2 - a * sum / (2 * (sum - a));
        if a >= b {
            break;
        }
        let c = sum - (a + b);

        if a * a + b * b == c * c {
            result.insert([a, b, c]);
        }
    }

    result
}
