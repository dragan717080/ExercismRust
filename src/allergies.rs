use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let allergies = Allergies::new(32);
    println!("{:?}", allergies.allergies());
}

pub struct Allergies {
    score: u32,
    allergen_scores:HashMap<Allergen, u32>
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

/**
 * When hashmap value is provided, find corresponding key.
 */
fn find_hashmap_key_by_value<'a, K, V>(map: &'a HashMap<K, V>, search_value: &V) -> Option<&'a K>
where
    K: Eq + Hash,
    V: PartialEq,
{
    for (k, v) in map.iter() {
        if v == search_value {
            return Some(k);
        }
    }
    None
}

/**
 * Get the highest degree of 2 that's less or equal to number e.g. 52 -> 5
 */
fn get_degree_of_two(n: &u32) -> u32 {
    let mut degree = 0;
    let mut v = 2;

    while v <= n.to_owned() {
        v *= 2;
        degree += 1;
    }

    degree
}

/**
 * Get all the degrees of 2 that can be sum within given range,
 * excluded k which is the result of 'get_degree_of_two'.
 * e.g. 127 is sum of all degrees from 0 to 6 and 37 is 4 and 1. (k == 5)
 * Iterates from 2*(k - 1) backwards until 2*0
 */
fn get_all_degrees_that_can_be_sum(n: &u32, k: u32) -> Vec<u32> {
    let mut degrees = Vec::new();

    if *n == 0 || k == 0 {
        return degrees;
    }

    let start_diff = n - 2_u32.pow(k);
    let mut total_diff = start_diff;

    let mut exp = (k - 1) as i32;

    while exp >= 0 {
        let pow_value = 2_i32.pow(exp.try_into().unwrap());
        if pow_value <= total_diff as i32 {
            total_diff -= pow_value as u32;
            degrees.push(pow_value as u32);
        }
        exp -= 1;
    }

    degrees
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score, allergen_scores: Allergies::get_allergen_scores() }
    }

    fn get_allergen_scores() -> HashMap<Allergen, u32> {
        HashMap::from([
            (Allergen::Eggs, 1),
            (Allergen::Peanuts, 2),
            (Allergen::Shellfish, 4),
            (Allergen::Strawberries, 8),
            (Allergen::Tomatoes, 16),
            (Allergen::Chocolate, 32),
            (Allergen::Pollen, 64),
            (Allergen::Cats, 128),
        ])
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let max_degree = get_degree_of_two(&self.score);
        let initial_allergy = find_hashmap_key_by_value(&self.allergen_scores, &2_u32.pow(max_degree));

        if self.score == 0 {
            return Vec::new();
        }

        let mut all_allergies: Vec<Allergen> = Vec::new();

        if initial_allergy.is_some() {
            all_allergies.push(initial_allergy.unwrap().clone());
        }

        for allergy_value in get_all_degrees_that_can_be_sum(&self.score, max_degree) {
            all_allergies.push(find_hashmap_key_by_value(&self.allergen_scores, &allergy_value).unwrap().clone());
        }

        all_allergies
    }
}
