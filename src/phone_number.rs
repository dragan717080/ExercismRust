fn main() {
    let s1 = "+1 (613)-995-0253";
    let s2 = "613-995-0253";
    let s3 = "1 613 995 0253";
    let s4 = "613.995.0253";

    println!("{:?}", number(s1));
    println!("{:?}", number(s2));
    println!("{:?}", number(s3));
    println!("{:?}", number(s4));
}

pub fn number(user_number: &str) -> Option<String> {
    let mut digits: Vec<u32> = user_number
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    println!("Digits: {:?}", digits);

    // Invalid number of digits
    if digits.len() < 10 || digits.len() > 11 {
        return None;
    }

    #[allow(unused_assignments)]
    let mut first_digit = None;

    // NANP numbers must start with 1
    if digits.len() == 11 {
        first_digit = Some(*digits.get(0).unwrap());
        if first_digit != Some(1) {
            return None;
        }

        digits = digits[1..digits.len()].to_vec();
    }

    // Format must be NXX NXX-XXXX
    if *digits.get(0).unwrap() < 2 || *digits.get(3).unwrap() < 2 {
        return None;
    }

    let digits_str = digits.iter().map(|d| d.to_string()).collect::<String>();

    Some(digits_str)
}
