use std::collections::HashMap;
use std::hash::Hash;

pub fn get_unique_combinations<T: Clone>(arr: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if arr.len() < k {
        return vec![];
    }

    let mut result = Vec::new();

    for i in 0..arr.len() {
        let mut sub_result = get_unique_combinations(&arr[i + 1..], k - 1);
        for sub_comb in sub_result.iter_mut() {
            sub_comb.insert(0, arr[i].clone());
        }
        result.append(&mut sub_result);
    }

    result
}

/**
 * Only finds one element.
 */
#[allow(dead_code)]
pub fn find_hashmap_by_value<'a, T1, T2>(map: &'a HashMap<T1, T2>, value: T2) -> (&'a T1, &'a T2)
    where
    T1: Eq + Hash,
    T2: PartialEq,
{
    map.iter().find(|&(_, v)| *v == value).unwrap()
}
