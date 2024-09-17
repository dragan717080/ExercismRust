fn main() {
    let s = "35";

    println!("{:?}", series(s, 2));
    let output = series(s, 2);
    let expected = &["35"];
    assert_eq!(output, expected);
}

pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();

    if digits.len() == 0 || len > digits.len() {
        return result;
    }

    for i in 0..(digits.len() - len + 1) {
        result.push(digits[i..i + len].chars().collect::<String>());
    }

    result
}
