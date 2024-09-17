fn main() {
    println!("{:?}", actions(19));
}

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result = Vec::new();
    let mut all_binary_digits = format!("{:b}", n).chars().collect::<Vec<char>>();

    let binary_digits = all_binary_digits.split_off(all_binary_digits.len().checked_sub(5).unwrap_or(0)).iter().map(|c| c.to_digit(2).unwrap()).collect::<Vec<u32>>();

    for (i, d) in binary_digits.iter().rev().enumerate() {
        if *d == 1 {
            match i {
                0 => result.push("wink"),
                1 => result.push("double blink"),
                2 => result.push("close your eyes"),
                3 => result.push("jump"),
                4 => result.reverse(),
                _ => ()
            }
        }
    }

    result
}
