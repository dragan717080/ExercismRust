fn main() {
    let input_base = 2;
    let input_digits = &[1, 0, 1];
    let output_base = 10;
    let output_digits = vec![5];
    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn to_decimal(number: &[u32], from_base: u32) -> u32 {
    let mut result: u32 = 0;

    let mut degree: u32 = 0;

    for digit in number.iter().rev() {
        result += digit * from_base.pow(degree);
        degree += 1;
    }

    result
}

/**
 * Returns map with exponent as key and count as value.
 * e.g. 2*10^4 = { 4: 2 }
 */
fn get_highest_degree(number: u32, to_base: u32) -> u32 {
    let mut degree = 0;
    let mut v = to_base;

    while v <= number.to_owned() {
        v *= to_base;
        degree += 1;
    }

    degree
}

fn decimal_to_base(number: u32, to_base: u32) -> Vec<u32> {
    // Highest exponent which is smaller than number in base
    // e.g. for number = 400 and to_base = 7 will give 3
    let mut n = number;

    // Get initial degree to know how much capacity to reserve in result
    let degree = get_highest_degree(n, to_base);

    let initial_degree = degree as usize + 1;
    let mut result = vec![0; initial_degree];

    while n > 0 {
        let degree = get_highest_degree(n, to_base);
        let quotient = n / to_base.pow(degree);
        let remainder = n - quotient * to_base.pow(degree);
        n = remainder;

        result[degree as usize] = quotient;
    }

    result.reverse();

    result
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    for digit in number {
        if !(*digit < from_base) {
            return Err(Error::InvalidDigit(*digit));
        }
    }

    let decimal_value = to_decimal(number, from_base);

    let result =  decimal_to_base(decimal_value, to_base);

    Ok(result)
}
