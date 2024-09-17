fn main() {
    let string_digits = "0123456789";
    let span = 5;

    println!("{:?}", lsp(string_digits, span));
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    let mut max_product: u64 = 0;

    for i in 0..=string_digits.len() - span {
        let digits = &string_digits[i..i + span];

        let product = digits.chars().into_iter().try_fold(1, |acc, c| {
            match c.to_digit(10) {
                Some(digit) => Ok(acc * digit as u64),
                None => Err(Error::InvalidDigit(c))
            }
        })?;

        if product > max_product {
            max_product = product;
        }
    }

    Ok(max_product)
}
