fn main() {
    let s = "										";

    println!("{}", reply(s));
}

pub fn reply(message: &str) -> &str {
    let is_silence = message.trim().len() == 0;

    if is_silence {
        return "Fine. Be that way!";
    }

    let message = message.trim();

    // Was it yell (all alphabetic chars are in uppercase)
    let alphabetic_chars = message.chars().filter(|c| c.is_alphabetic()).collect::<Vec<char>>();
    let is_yell = alphabetic_chars.len() > 0 && alphabetic_chars.into_iter().all(|c| c.is_uppercase());

    let is_question = message.chars().last().unwrap() == '?';

    if is_question {
        // Was it yell
        if is_yell {
            return "Calm down, I know what I'm doing!";
        }

        return "Sure.";
    }

    if is_yell {
        return "Whoa, chill out!";
    }

    "Whatever."
}
