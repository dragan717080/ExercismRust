use std::collections::HashSet;

fn main() {
    let input = 4;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    if num == 1 {
        return Some(Classification::Deficient);
    }

    let sum_of_factors: u64 = get_factors(num).iter().sum();

    if num > sum_of_factors {
        return Some(Classification::Deficient);
    } else if sum_of_factors == num {
        return Some(Classification::Perfect);
    }

    Some(Classification::Abundant)
}

/**
 * Get factors of given number (excluding number itself).
 */
fn get_factors(num: u64) -> HashSet<u64> {
    let mut result = HashSet::from([1]);
    let upper_bound = (num as f64).sqrt() as u64 + 1;

    for i in 2..upper_bound {
        if num % i == 0 {
            result.insert(i);
            result.insert(num / i);
        }
    }

    result
}
