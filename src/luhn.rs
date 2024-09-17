fn main() {
    let s1 = "095 245 88";
    println!("{}", is_valid(s1));

    let s2 = "055 444 286";
    println!("{}", is_valid(s2));
}

pub fn is_valid(code: &str) -> bool {
    if !code.chars().all(|c| c.is_digit(10) || c == ' ') {
        return false;
    }

    let reversed_code = code.split(" ").collect::<String>().chars().rev().collect::<String>();

    if reversed_code.len() < 2 {
        return false;
    }

    let mut result = String::new();

    for (i, c) in reversed_code.char_indices().rev() {
        if i % 2 != 0 {
            let mut d = c.to_digit(10).unwrap() * 2;

            if d > 9 {
                d -= 9;
            }

            result += &d.to_string();
        } else {
            result += &c.to_string();
        }
    }

    result.chars().fold(0, |acc, c| acc + c.to_digit(10).unwrap()) % 10 == 0
}
