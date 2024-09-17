fn main() {
    let input = 89;
    let output = egg_count(input);
    let expected = 4;
    assert_eq!(output, expected);
}

pub fn egg_count(decimal: u32) -> usize {
    let binary = format!("{:b}", decimal);

    binary.chars().filter(|d| *d == '1').collect::<Vec<char>>().len()
}
