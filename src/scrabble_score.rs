use std::collections::HashMap;

fn main() {

}

pub fn score(word: &str) -> u64 {
    let mut scores: HashMap<u32, Vec<&str>> = HashMap::new();

    scores.insert(1, vec!["A", "E", "I", "O", "U", "L", "N", "R", "S", "T"]);
    scores.insert(2, vec!["D", "G"]);
    scores.insert(3, vec!["B", "C", "M", "P"]);
    scores.insert(4, vec!["F", "H", "V", "W", "Y"]);
    scores.insert(5, vec!["K"]);
    scores.insert(8, vec!["J", "X"]);
    scores.insert(10, vec!["Q", "Z"]);

    // Convert to chars to handle ASCII characters being interpreted correctly
    let word_upper_chars = word.chars().into_iter().map(|c| c.to_uppercase().to_string());

    let score = &word_upper_chars.into_iter().fold(0, |acc, c| {
        // Method 'unwrap_or_default' handles non ASCII characters
        let score_for_char = get_key_for_char(&scores, &c.to_string()).unwrap_or_default();
        acc + score_for_char
    });

    *score as u64
}

/**
 * Gets the corresponding key for character in hashmap.
 */
fn get_key_for_char(map: &HashMap<u32, Vec<&str>>, value: &str) -> Option<u32> {
    map.iter()
        .find_map(|(key, val)| if val.contains(&value) { Some(key.clone()) } else { None })
}
