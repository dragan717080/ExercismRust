extern crate itertools;

fn main() {
    let s = "A + A + A + A + A + A + A + A + A + A + A + B == BCC";
    println!("{:?}", solve(s));
}

use std::char;
use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

fn test_equation(
    coefficients: &HashMap<char, i64>,
    cant_be_zero: &HashSet<char>,
    substitutions: &HashMap<char, u8>,
) -> bool {
    if cant_be_zero.iter().any(|d| substitutions[d] == 0) {
        return false;
    }

    coefficients
        .iter()
        .map(|(d, &coeff)| i64::from(substitutions[d]) * coeff)
        .sum::<i64>()
        == 0
}

fn get_coefficients(puzzle: &str) -> (HashMap<char, i64>, HashSet<char>) {
    let mut coefficients = HashMap::new();
    let mut cant_be_zero = HashSet::new();
    let equation: Vec<&str> = puzzle.split("==").collect();

    let insert_term = |term: &str, polarity: i64, coefficients: &mut HashMap<char, i64>, cant_be_zero: &mut HashSet<char>| {
        cant_be_zero.insert(term.chars().next().unwrap());
        let mut power_of_ten = polarity;
        for letter in term.chars().rev() {
            *coefficients.entry(letter).or_insert(0) += power_of_ten;
            power_of_ten *= 10;
        }
    };

    equation[0].split('+').map(str::trim).for_each(|left_term| insert_term(left_term, 1, &mut coefficients, &mut cant_be_zero));
    insert_term(equation[1].trim(), -1, &mut coefficients, &mut cant_be_zero);

    (coefficients, cant_be_zero)
}

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let (coefficients, cant_be_zero) = get_coefficients(puzzle);
    let letters: Vec<char> = coefficients.keys().cloned().collect();
    let numbers: Vec<u8> = (0..10).collect();

    for combination in numbers.iter().combinations(letters.len()) {
        let comb = combination.clone();
        for permutation in comb.iter().permutations(letters.len()) {
            let substitution: HashMap<char, u8> = letters.iter().zip(permutation.iter().cloned()).map(|(&c, &n)| (c, *n)).collect();
            if test_equation(&coefficients, &cant_be_zero, &substitution) {
                return Some(substitution);
            }
        }
    }

    None
}
