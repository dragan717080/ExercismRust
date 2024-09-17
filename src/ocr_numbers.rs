use std::collections::HashSet;

fn main() {
    let input = 
    "    _  _ \n".to_string() +
    "  | _| _|\n" +
    "  ||_  _|\n" +
    "         \n" +
    "    _  _ \n" +
    "|_||_ |_ \n" +
    "  | _||_|\n" +
    "         \n" +
    " _  _  _ \n" +
    "  ||_||_|\n" +
    "  ||_| _|\n" +
    "         ";

    assert_eq!(Ok("123,456,789".to_string()), convert(&input));
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let all_lines = input.split("\n").map(|line| line.to_string()).collect::<Vec<String>>();

    if all_lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(all_lines.len()));
    }

    let numbers_lines = all_lines.chunks(4).map(|chunk| chunk.to_vec()).collect::<Vec<Vec<String>>>();

    let mut commas_count = numbers_lines.len() - 1;

    let mut result = String::new();

    for number_line in numbers_lines {
        process_number_line(number_line, &mut result)?;

        if commas_count > 0 {
            result += ",";
            commas_count -= 1;
        }
    }

    Ok(result)
}

fn process_number_line(number_line: Vec<String>, result: &mut String) -> Result<(), Error> {
    for line in &number_line {
        if line.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line.len()));
        }
    }

    let potential_numbers = get_potential_numbers(&number_line[0]);

    let number_line_str = process_other_lines(number_line[1..3].to_vec(), &potential_numbers);

    *result += number_line_str.as_ref();

    Ok(())
}

/**
 * Get potential numbers from the first line.
 * 
 * 1 and 4 have no '_' at the top, others have.
 */
fn get_potential_numbers(first_line: &String) -> Vec<Vec<u8>> {
    let first_line_chunks: Vec<String> = first_line.chars().collect::<Vec<char>>().chunks(3).map(|chunk| chunk.iter().collect::<String>()).collect();

    let first_line_chunks: Vec<&str> = first_line_chunks.iter().map(|s| s.as_str()).collect();

    let mut potential_numbers: Vec<Vec<u8>> = Vec::with_capacity(first_line_chunks.len());

    for (i, chunk) in first_line_chunks.iter().enumerate() {
        match *chunk {
            "   " => {
                if potential_numbers.len() > i {
                    potential_numbers[i].extend(vec![1, 4]);
                } else {
                    potential_numbers.push(vec![1, 4]);
                }
            },
            " _ " => {
                if potential_numbers.len() > i {
                    potential_numbers[i].extend(vec![0, 2, 3, 5, 6, 7, 8, 9]);
                } else {
                    potential_numbers.push(vec![0, 2, 3, 5, 6, 7, 8, 9]);
                }
            }
            _ => ()
        };
    }

    potential_numbers
}

/**
 * From the list of potential numbers, get the one that is possible after processing lines 2 and 3.
 * 
 * Returns string representation of a digit, otherwise '?'.
 */
fn process_other_lines(number_line: Vec<String>, potential_numbers: &Vec<Vec<u8>>) -> String {
    let second_line_chunks: Vec<String> = number_line[0].chars().collect::<Vec<char>>().chunks(3).map(|chunk| chunk.iter().collect::<String>()).collect();

    let second_line_chunks: Vec<&str> = second_line_chunks.iter().map(|s| s.as_str()).collect();

    let third_line_chunks: Vec<String> = number_line[1].chars().collect::<Vec<char>>().chunks(3).map(|chunk| chunk.iter().collect::<String>()).collect();

    let third_line_chunks: Vec<&str> = third_line_chunks.iter().map(|s| s.as_str()).collect();

    let mut combined_lines: Vec<String> = Vec::new();

    let mut result = String::new();

    for (i, chunk) in second_line_chunks.iter().enumerate() {
        combined_lines.push(chunk.to_string() + third_line_chunks[i]);
    }

    for (i, combined_line) in combined_lines.iter().enumerate() {
        let possible_values = match combined_line.as_ref() {
            "| ||_|" => vec![0],
            "  |  |" => vec![1, 7],
            " _||_ " => vec![2],
            " _| _|" => vec![3],
            "|_|  |" => vec![4],
            "|_  _|" => vec![5],
            "|_ |_|" => vec![6],
            "|_||_|" => vec![8],
            "|_| _|" => vec![9],
            _ => vec![],
        };

        let potential_numbers: HashSet<&u8, _> = HashSet::<&u8>::from_iter(potential_numbers[i].iter());
        let possible_values: HashSet<&u8, _> = HashSet::<&u8>::from_iter(possible_values.iter());
        let value = potential_numbers.intersection(&possible_values).map(|value| **value).collect::<Vec<u8>>();

        if let Some(value) = value.get(0) {
            result += &value.to_string();
        } else {
            result += "?";
        }
    }

    result
}
