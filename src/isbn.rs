fn main() {
    let s = "359821507X";
    
    println!("{}", is_valid_isbn(s));
}

pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits = isbn.chars().into_iter().filter(|c| c.is_alphanumeric()).collect::<Vec<char>>();

    if !(digits.len() == 10) {
        return false;
    }

    let (digits, check) = digits.split_at(digits.len() - 1);

    if !digits.iter().all(|c| c.is_digit(10)) {
        return false;
    }

    let check = check.get(0).unwrap();

    if !check.is_digit(10) && *check != 'X' {
        return false;
    }

    let mut sum: u32 = 0;

    for i in (2..11).rev() {
        let digit = digits.get(10 - i).unwrap().to_digit(10).unwrap();
        sum += i as u32 * digit;
    }

    let last_digit = check.to_digit(10).unwrap_or(10);

    sum += last_digit;
 
    sum % 11 == 0
}
