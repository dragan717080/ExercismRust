fn main() {
    let input = "What is -3 multiplied by 25?";
    let output = answer(input);
    let expected = Some(-75);
    assert_eq!(output, expected);
}

pub fn answer(command: &str) -> Option<i32> {
    let mut words = command.split_whitespace().skip(2).map(|word| word.to_string()).collect::<Vec<String>>();

    if let Some(last_word) = words.last_mut() {
        *last_word = last_word.replace("?", "");
    }

    // Handle exponents
    for (i, word) in words.clone().iter().enumerate() {
        if word == "power" {
            words[i - 1] = words[i - 1].chars().filter(|c| c.is_digit(10)).collect();
            words.remove(i);
        }
    }

    let numbers = words.iter().filter(|word| word.parse::<i32>().is_ok()).map(|d| d.parse().unwrap()).collect::<Vec<i32>>();

    return match numbers.len() {
        1 => {
            match words.len() {
                1 => Some(numbers[0]),
                _ => None
            }
        },
        2 => handle_operation(&words),
        3 => handle_two_operations(&words),
        _ => None
    };
}

fn handle_operation(words: &[String]) -> Option<i32> {
    // Words denoting operation e.g. 'multiplied by'
    let [a, b]: [i32; 2] = [words[0].parse().ok()?, words.last()?.parse().ok()?];
    let operation_words = words[1..words.len() - 1].iter().map(|word| word.as_str()).collect::<Vec<&str>>();
 
    if operation_words.len() == 0 {
        return None;
    }

    let result = match operation_words[0] {
        "plus" => {
            if operation_words.len() == 1 { a + b } else { None? }
        },
        "minus" => {
            if operation_words.len() == 1 { a - b } else { None? }
        },
        "multiplied" => {
            if operation_words.len() == 2 { a * b } else { None? }
        },
        "divided" => {
            if operation_words.len() == 2 { a / b } else { None? }
        },
        "raised" => {
            if operation_words.len() == 6 { a.pow(b as u32) } else { None? }
        },
        _ => panic!("Operation {} is unknown", operation_words.join(" ")),
    };

    Some(result)
}

fn handle_two_operations(words: &[String]) -> Option<i32> {
    let mut second_number_index: usize = 0;

    for (i, word) in words.iter().skip(1).enumerate() {
        if word.parse::<i32>().is_ok() {
            second_number_index = i + 1;
            break;
        }
    }

    // Result of intermediate operation
    let first_operation_result = handle_operation(&words[0..second_number_index + 1]);

    match first_operation_result {
        Some(first_operation_result) => {
            let mut second_operation_words = vec![first_operation_result.to_string()];

            second_operation_words.extend_from_slice(&words[(second_number_index + 1)..words.len()]);
        
            return handle_operation(&second_operation_words);
        },
        None => None
    }
}
