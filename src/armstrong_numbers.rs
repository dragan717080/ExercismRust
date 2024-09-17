fn main() {
    let n1 = 153;
    let n2 = 154;

    println!("{}", is_armstrong_number(n1));
    println!("{}", is_armstrong_number(n2));
    println!("{}", is_armstrong_number(4_106_098_957));
}

pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let e = num_str.len();

    let mut sum = 0;

    for letter in num_str.chars() {
        let digit = letter.to_digit(10).unwrap();
        let check_pow = digit.checked_pow(e as u32);

        match check_pow {
            Some(pow_result) => {
                let check_sum = (sum as u32).checked_add(pow_result);

                match check_sum {
                    Some(new_sum) => sum = new_sum,
                    None => {
                        println!("Overflow occured while adding {}", sum);
                        return false;
                    }
                }
            }
            None => {
                println!("Overflow occured while raising {} to the power of {}", num, e);
                return false;
            }
        }
    }

    sum == num
}
