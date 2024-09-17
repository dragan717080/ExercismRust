fn main() {
    let text = "Testing 1 2 3 testing";
    let shift_key = 4;
    let output = rotate(text, shift_key);
    let expected = "Xiwxmrk 1 2 3 xiwxmrk";
    assert_eq!(output, expected);
}

pub fn rotate(input: &str, key: u8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if !c.is_alphabetic() {
            result += &c.to_string();
            continue;
        }

        let letter_code = c as u32;

        // 'a' lowercase, 'A' uppercase
        let start_code = if c.is_uppercase() { 65 } else { 97 };

        let letter_code = start_code + (letter_code as u8 + key - start_code) % 26;

        result += &char::from_u32(letter_code as u32).unwrap().to_string();
    }

    result
}
