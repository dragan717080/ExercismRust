fn main() {
    let input = "XYZ";
    let output = encode(input);
    let expected = "XYZ";
    assert_eq!(output, expected);
}

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let s_chars = source.chars().collect::<Vec<char>>();

    for (i, c) in source.char_indices().skip(1) {
        let prev_char = s_chars[i - 1];

        if c != prev_char || i == s_chars.len() - 1 {
            let mut count = 1;

            for j in (1..i).rev() {
                if s_chars[j] == s_chars[j - 1] {
                    count += 1;
                } else {
                    break;
                }
            }

            if i == s_chars.len() - 1 && c == prev_char {
                count += 1;
            }

            if count > 1 {
                result += &count.to_string();
            }

            result += &prev_char.to_string();

            if i == s_chars.len() - 1 && c != prev_char {
                result += &s_chars.last().unwrap().to_string();
            }
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let s_chars = source.chars().collect::<Vec<char>>();

    for (i, c) in source.char_indices() {
        if !c.is_numeric() {
            let mut count = 1;

            if i != 0 {
                count = get_count(i, &s_chars);
            }

            for _ in 0..count {
                result += &c.to_string();
            }
        }
    }

    result
}

fn get_count(i: usize, s_chars: &Vec<char>) -> u32 {

    let mut num_str = String::new();

    for j in (0..i).rev() {
        if !s_chars[j].is_numeric() {
            break;
        }

        num_str.insert(0, s_chars[j]);
    }

    let num = num_str.parse().unwrap_or(1) as u32;

    num
}
