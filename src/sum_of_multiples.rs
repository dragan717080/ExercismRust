use std::collections::HashSet;

fn main() {
    let factors = &[3, 5];
    let limit = 1000;
    
    println!("{}", sum_of_multiples(limit, factors)); // 233168
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();

    for i in 1..limit {
        for factor in factors {
            if factor == &0 {
                continue;
            }

            if &i >= factor && i % factor == 0 {
                multiples.insert(i);
            }
        }
    }

    multiples.iter().sum()
}
