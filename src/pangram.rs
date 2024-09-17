use std::collections::HashSet;

fn main() {
    let s1 = "The quick brown fox jumps over the lazy dog";
    let s2 = "abcdefghijklm ABCDEFGHIJKLM";

    println!("{}", is_pangram(s1));
    println!("{}", is_pangram(s2));
}

pub fn is_pangram(sentence: &str) -> bool {
    let mut letters_codes: HashSet<u32> = HashSet::new();
    for c in sentence.to_uppercase().chars() {
        letters_codes.insert(c as u32);
    }

    let uppercase_letters_count = letters_codes.iter().filter(|&&d| d > 64 && d < 91).collect::<Vec<&u32>>().len();

    uppercase_letters_count == 26
}
