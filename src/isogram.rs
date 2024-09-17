use std::collections::HashSet;

fn main() {
    let s = "background";

    println!("{}", check(s));
}

pub fn check(candidate: &str) -> bool {
    let mut unique_letters: HashSet<char> = HashSet::new();

    for c in candidate.chars() {
        let c = c.to_lowercase().next().unwrap() as char;

        if c.is_alphabetic() && c.is_lowercase() {
            if unique_letters.contains(&c) {
                return false;
            }

            unique_letters.insert(c);
        }
    }

    true
}
