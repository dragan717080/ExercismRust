fn main() {
    let output = palindrome_products(100, 999).map(|(_, max)| max.into_inner());

    println!("{:?}", output);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        let value_str = value.to_string();

        if value_str == value_str.chars().rev().collect::<String>() {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_palindrome: Option<Palindrome> = None;
    let mut max_palindrome: Option<Palindrome> = None;

    for i in min..max + 1 {
        for j in min..i + 1 {
            let product = i * j;

            let palindrome = Palindrome::new(product);

            if palindrome.is_some() {
                if min_palindrome.is_none() {
                    min_palindrome = Some(palindrome.unwrap());
                    // Assign max palindrome initially right after finding min
                    // If larger palindrome is detected, it will become new max
                    max_palindrome = Some(palindrome.unwrap());
                }

                if product > Palindrome::into_inner(max_palindrome?) {
                    max_palindrome = Some(palindrome.unwrap());
                }
            }
        }
    }

    match min_palindrome {
        Some(_) => Some((min_palindrome?, max_palindrome?)),
        None => None
    }
}
