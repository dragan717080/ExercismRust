use std::collections::HashMap;


fn main() {
    let input = "Joe can't tell between 'large' and large.";
    let mut output = word_count(input);
    let expected = [
        ("joe", 1),
        ("can't", 1),
        ("tell", 1),
        ("between", 1),
        ("large", 2),
        ("and", 1),
    ];
    for (word, count) in expected {
        assert_eq!((word, output.remove(word).unwrap_or(0)), (word, count));
    }
    assert_eq!(output.into_iter().collect::<Vec<_>>(), vec![]);
}

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut words = words.split([' ', ',', '\n', '"', ':', '!', '&', '@', '$', '%', '^', '&', '.']).map(|word| word.to_string()).collect::<Vec<String>>();

    words = words.into_iter().map(|word| word.to_lowercase()).collect();
    // Remove empty splits and apostrophes at the end of the word
    words = words.into_iter().filter(|word| *word != "" && *word != "'").collect();

    // Remove empty splits, also remove quotations that are at beginning or at the end of the word (e.g. "Shouldn't " is ok)
    words = words.into_iter().map(|mut word| {
        let letters = word.chars().collect::<Vec<char>>();
        if *letters.first().unwrap() == '\'' {
            word = word[1..word.len()].to_string();
        }

        if *letters.last().unwrap() == '\'' {
            word = word[0..word.len() - 1].to_string();
        }

        word
    }).collect();

    let mut result: HashMap<String, u32> = HashMap::new();

    for word in words {
        *result.entry(word.to_string()).or_insert(0) += 1;
    }

    result
}
