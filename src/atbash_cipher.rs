fn main() {
    assert_eq!(
        decode("zmlyh gzxov rhlug vmzhg vkkrm thglm v"),
        "anobstacleisoftenasteppingstone"
    );
}

fn tranpose(ch: char) -> char {
    if ch.is_ascii_digit() { ch } else { ('z' as u8 - ch as u8 + 'a' as u8) as char
    }
}

pub fn encode(plaintext: &str) -> String {
    plaintext
        .to_lowercase()
        .chars()
        .filter(|&ch| ch.is_ascii())
        .filter(|&ch| ch.is_alphanumeric())
        .map(tranpose)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|slice| slice.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(ciphertext: &str) -> String {
    ciphertext
        .split::<char>(' ')
        .collect::<String>()
        .chars()
        .map(tranpose)
        .collect::<String>()
}
