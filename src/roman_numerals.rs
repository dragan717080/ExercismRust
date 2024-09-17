use std::fmt::{Display, Formatter, Result};

fn main() {
    let input = 3999;
    let output = Roman::from(input).to_string();
    let expected = "MMMCMXCIX";
    assert_eq!(output, expected);
}

static NUMERALS: [(usize, &str); 13] = [
    (1, "I"),
    (4, "IV"),
    (5, "V"),
    (9, "IX"),
    (10, "X"),
    (40, "XL"),
    (50, "L"),
    (90, "XC"),
    (100, "C"),
    (400, "CD"),
    (500, "D"),
    (900, "CM"),
    (1000, "M"),
];

pub struct Roman {
    num: usize,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut start = self.num;
        let mut result = String::new();
        for &(numeric, roman_string) in NUMERALS.iter().rev() {
            while start >= numeric {
                result.push_str(roman_string);
                start -= numeric;
            }
        }
        write!(f, "{}", result)
    }
}

impl From<usize> for Roman {
    fn from(num: usize) -> Self {
        Self { num }
    }
}
