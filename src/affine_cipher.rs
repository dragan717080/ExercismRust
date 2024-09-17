fn main() {
    let phrase = "odpoz ub123 odpoz ub";
    let (a, b) = (25, 7);
    let output = decode(phrase, a, b);
    let expected = Ok("testing123testing".into());
    assert_eq!(output, expected);
}

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: u32, b: u32) -> Result<String, AffineCipherError> {
    let plaintext = plaintext.replace([' ', '.', ','], "");

    let mut result = String::new();
    let mut result_len = 0;

    let modular_inverse = modulo_inverse(a as i32, 26 as i32);

    if modular_inverse.is_none() {
        return Err(AffineCipherError::NotCoprime(a as i32));
    }

    for mut c in plaintext.chars() {
        if c.is_alphabetic() {
            c = c.to_lowercase().next().unwrap();
            let char_code = c as u32;

            if char_code > 96 && char_code < 123 {
                // Index in alphabet (0 starting)
                let i =  c as u32 - 97;
                let e = (a * i + b) % 26;

                if result_len > 0 && result_len % 5 == 0 {
                    result += " ";
                }

                let encoded_char = char::from_u32(97 + e).unwrap();

                result += &encoded_char.to_string();
            }
        } else {
            result += &c.to_string();
        }

        result_len += 1;
    }

    Ok(result)
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let ciphertext = ciphertext.replace([' ', '.', ','], "");
    let modular_inverse = modulo_inverse(a as i32, 26 as i32);

    let mut result = String::new();

    match modular_inverse {
        Some(modular_inverse) => {
            for mut c in ciphertext.chars() {
                if c.is_alphabetic() {
                    c = c.to_lowercase().next().unwrap();
                    let char_code = c as u32;

                    if char_code > 96 && char_code < 123 {
                        // Index in alphabet (0 starting)
                        let i =  c as i32 - 97;

                        let product = modular_inverse * (i - b);

                        // Must use 'rem_euclid' to handle negative products
                        let decripted = product.rem_euclid(26) as u32;
                        let decripted_char = char::from_u32(97 + decripted).unwrap();

                        result += &decripted_char.to_string();
                    }
                } else {
                    result += &c.to_string();
                }
            }
        },
        None => {
            return Err(AffineCipherError::NotCoprime(a as i32));
        }
    }

    Ok(result)
}

/**
 * Finds GCD alongside the coeficients x and y 
 */
fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/**
 * Use Euclid's extended algorithm to find modulo inverse between two numbers
 (if they are coprime).
 */
fn modulo_inverse(a: i32, m: i32) -> Option<i32> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None // No modular inverse exists if gcd(a, m) != 1
    } else {
        // Ensure x is positive
        Some(((x % m + m) % m) as i32)
    }
}
