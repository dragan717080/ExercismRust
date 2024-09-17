fn main() {
    let s1 = "Liquid-crystal display";
    let s2 = "HyperText Markup Language";
    let s3 = "The Road _Not_ Taken";

    println!("{}", abbreviate(s1));
    println!("{}", abbreviate(s2));
    println!("{}", abbreviate(s3));
}

pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let phrase = phrase.trim().split("-").collect::<Vec<&str>>();
    println!("{:?}", phrase);

    for word in phrase {
        let new_words = word.split(" ").collect::<Vec<&str>>();

        for new_word in new_words {
            let new_word: String = new_word.chars()
                .filter(|c| c.is_alphanumeric() || ['\'', '"', '!', '.'].contains(c))
                .collect();

            // Add first letter or all uppercase letters to the acronym
            for (i, c) in new_word.chars().into_iter().enumerate() {
                if i == 0 || c.is_uppercase() && !new_word.chars().into_iter().all(|c| c.is_uppercase()) {
                    acronym += &c.to_string().to_uppercase();
                }
            }
        }
    }

    acronym
}
