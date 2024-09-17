use std::collections::{HashMap, HashSet};

fn main() {
    let word = "ΑΒΓ";

    let candidates = ["αβγ", "ΒΓΑ", "γβα"];

    println!("{:?}", anagrams_for(word, &candidates));
}

/**
 * Is word anagram of another word.
 */
fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    // Word is not an anagram of itself
    if word1 == word2 {
        return false;
    }

    let mut word1_letter_freqs: HashMap<char, u32> = HashMap::new();
    let mut word2_letter_freqs: HashMap<char, u32> = HashMap::new();

    for letter in word1.chars() {
        *word1_letter_freqs.entry(letter).or_insert(1) += 1;
    }

    for letter in word2.chars() {
        *word2_letter_freqs.entry(letter).or_insert(1) += 1;
    }

    word1_letter_freqs == word2_letter_freqs
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for candidate in possible_anagrams {
        if is_anagram(&word.to_lowercase(), &candidate.to_lowercase()) {
            anagrams.insert(&candidate);
        }
    }

    anagrams
}
