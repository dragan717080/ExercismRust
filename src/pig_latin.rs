#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

fn main() {

}

pub fn translate_word(word: &str) -> String {
    lazy_static! {
        static ref VOWEL: Regex = Regex::new(r"^([aeiou]|y[^aeiou]|xr)[a-z]*").unwrap();
        static ref CONSONANTS: Regex = Regex::new(r"^([^aeiou]?qu|[^aeiou][^aeiouy]*)([a-z]*)").unwrap();
    }

    if VOWEL.is_match(word) {
        String::from(word) + "ay"
    } else {
        let caps = CONSONANTS.captures(word).unwrap();
        String::from(&caps[2]) + &caps[1] + "ay"
    }
}

pub fn translate(text: &str) -> String {
    text.split(' ')
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}
