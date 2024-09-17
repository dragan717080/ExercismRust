fn main() {
    println!("{}", encrypt("congratulate"));
}

pub fn encrypt(input: &str) -> String {
    if input.len() == 0 {
        return input.to_string();
    }

    let s = String::from(input.replace([' ', ',', '.', '\n', '-'], "").to_lowercase());

    #[allow(unused_variables)]
    let [rows, cols] = find_square_dimensions(s.len() as u64);

    let a = s.chars().collect::<Vec<char>>().chunks(cols as usize).map(|chunk| chunk.to_vec()).collect::<Vec<Vec<char>>>();

    let last_element = a.last().unwrap();
    let last_element_mut = &mut last_element.clone();

    // Assuming to_extend is calculated somewhere
    let to_extend = cols as usize - last_element.len();

    // Extend the last element with spaces
    last_element_mut.extend(&vec![' '; to_extend]);

    // If you need to update the last element in matrix
    let mut a = a;
    if let Some(last) = a.last_mut() {
        *last = last_element_mut.clone();
    }

    let transposed = transpose(a);

    let result = transposed.iter().map(|word_chars| word_chars.iter().collect::<String>()).collect::<Vec<String>>().join(" ");

    result
}

fn get_factors(num: u64) -> Vec<u64> {
    let mut result = Vec::new();

    let upper_bound = (num as f64).sqrt() as u64 + 1;

    for i in 1..upper_bound {
        if num % i as u64 == 0 {
            result.extend(&vec![i, num / i as u64]);
        }
    }

    result.sort();

    result.reverse();

    result
}

/**
 * Finds two numbers, m and k, such that their product is larger than num
 * and their difference is 1.
 * 
 * e.g. for 54 will be 8 and 7.
 */
fn find_square_dimensions(num: u64) -> [u64; 2] {
    let mut current = num;

    loop {
        let factors = get_factors(current);

        for i in 0..factors.len() - 1 {
            let current_factor = factors[i];
            let next_factor = factors[i + 1];

            if current_factor * next_factor < num {
                continue;
            }

            if current_factor - next_factor <= 1 {
                return [next_factor, current_factor];
            }
        }

        current += 1;
    }
}

fn transpose<T: Clone + Default>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut res: Vec<Vec<T>> = vec![vec![Default::default(); rows]; cols];

    for i in 0..cols {
        for j in 0..rows {
            res[i][j] = matrix[j][i].clone();
        }
    }

    res
}
