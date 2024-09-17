use std::collections::HashMap;

fn main() {
    println!("{}", encode(12323232));
    println!("{}", encode(15423));
    println!("{}", encode(14));
    println!("{}", encode(987654321123));
    println!("{}", encode(900600325123));
}

pub fn encode(n: u64) -> String {
    let mut numbers: HashMap<u64, &str> = HashMap::new();

    numbers.insert(0, "zero");
    numbers.insert(1, "one");
    numbers.insert(2, "two");
    numbers.insert(3, "three");
    numbers.insert(4, "four");
    numbers.insert(5, "five");
    numbers.insert(6, "six");
    numbers.insert(7, "seven");
    numbers.insert(8, "eight");
    numbers.insert(9, "nine");
    numbers.insert(10, "ten");
    numbers.insert(11, "eleven");
    numbers.insert(12, "twelve");
    numbers.insert(13, "thirteen");
    numbers.insert(14, "fourteen");
    numbers.insert(15, "fifteen");
    numbers.insert(16, "sixteen");
    numbers.insert(17, "seventeen");
    numbers.insert(18, "eighteen");
    numbers.insert(19, "nineteen");
    numbers.insert(20, "twenty");
    numbers.insert(30, "thirty");
    numbers.insert(40, "forty");
    numbers.insert(50, "fifty");
    numbers.insert(60, "sixty");
    numbers.insert(70, "seventy");
    numbers.insert(80, "eighty");
    numbers.insert(90, "ninety");
    numbers.insert(100, "hundred");
    numbers.insert(1_000, "thousand");
    numbers.insert(1_000_000, "million");
    numbers.insert(1_000_000_000, "billion");
    numbers.insert(1_000_000_000_000, "trillion");
    numbers.insert(1_000_000_000_000_000, "quadrillion");
    numbers.insert(1_000_000_000_000_000_000, "quintillion");

    let number_in_words = print_number_in_words(n, numbers);

    number_in_words
}

fn print_number_in_words(n: u64, numbers: HashMap<u64, &str>) -> String {
    let mut number = n.clone();

    let mut number_in_words = String::new();

    let mut keys: Vec<_> = numbers.keys().collect();
    keys.sort();
    keys.reverse();

    for k in keys {
        let quantity = number / *k;
        let mut quantity_str = "".to_string();
        if k >= &100u64 {
            let quantity_option = numbers.get(&quantity);
            match quantity_option {
                // e.g. 'one' in 'one thousand'
                Some(_) => {
                    quantity_str = numbers.get(&quantity).unwrap().to_string();
                }
                None => {
                    // It is in hundreds e.g. 987
                    if quantity >= 100u64 {
                        let hundreds = quantity / 100;
                        quantity_str = format!("{} hundred", numbers.get(&hundreds).unwrap());

                        let _remainder_str = "";
                        let remainder = quantity - hundreds * 100;
                        if remainder != 0 {
                            quantity_str += " ";
                            let new_value = get_words_for_small_number(numbers.clone(), remainder, &mut quantity_str);
                            // Prevent double add
                            let to_add = quantity_str.split(" ").collect::<Vec<&str>>().len() > 3;

                            if to_add {
                                quantity_str += &new_value;
                            }
                        }
                    } else {
                        quantity_str = get_words_for_small_number(numbers.clone(), quantity, &mut quantity_str);
                    }
                }
            }
        } else {
            return get_words_for_small_number(numbers.clone(), number, &mut number_in_words);
        }

        if number > *k {
            number_in_words += &format!("{} {} ", quantity_str, numbers.get(&k).unwrap());
            number -= (number / *k) * *k;
            if number == 0 {
                number_in_words = number_in_words.to_string().trim().to_string();
                break;
            }
        } else if number == *k {
            number_in_words += &format!("{} {}", quantity_str, numbers.get(&k).unwrap());
            break;
        }
    }

    number_in_words
}

/**
 * Handle numbers under 100.
 */
fn get_words_for_small_number(numbers: HashMap<u64, &str>, number: u64, number_in_words: &mut String) -> String {
    if number > 19 {
        let decs = (number / 10 ) * 10;
        let mut decs_str = numbers.get(&decs).unwrap().to_string();
        let singles = number - decs;
        let mut singles_str = "";
        if singles != 0 {
            singles_str = numbers.get(&singles).unwrap();
        }
        // If number is 20, don't make it with hyphen e.g. 'twenty-'
        if singles_str != "" {
            decs_str += "-";
        }
        number_in_words.push_str(&format!("{}{}", decs_str, singles_str));
    } else {
        number_in_words.push_str(&numbers.get(&number).unwrap().to_string());
    }

    number_in_words.to_owned()
}
