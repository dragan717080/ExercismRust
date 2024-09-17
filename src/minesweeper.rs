fn main() {
    let minefield = [];

    println!("{:?}", annotate(&minefield));
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let rows = minefield.len();

    if minefield.len() == 0 {
        return result;
    }

    let cols = minefield[0].chars().collect::<Vec<char>>().len();

    for i in 0..rows {
        result.push(String::new());
        for j in 0..cols {
            let c = annotate_single_field(minefield, i, j, rows, cols);
            result[i].push(c);
        }
    }

    result
}

fn annotate_single_field(minefield: &[&str], i: usize, j: usize, rows: usize, cols: usize) -> char {
    let mut neighbor_bombs = 0;

    let current_element = minefield[i].chars().nth(j).unwrap() as char;

    if current_element == '*' {
        return '*';
    }

    for row in (i as i32 - 1)..(i as i32 + 2) {
        if row < 0 || row == rows.try_into().unwrap() {
            continue;
        }
        for col in (j as i32 - 1)..(j as i32 + 2) {
            // Cases when current row/col is first or last
            if col < 0 || col == cols.try_into().unwrap() {
                continue;
            }

            // Ignore the current element when determining neighbors
            if row == i.try_into().unwrap() && col == j.try_into().unwrap() {
                continue;
            }
            let neighbor_row = row as usize;

            let neighbor = minefield[neighbor_row]
                .chars()
                .nth(col.try_into().unwrap())
                .unwrap_or('0');

            if neighbor == '*' {
                neighbor_bombs += 1
            }
        }
    }

    if neighbor_bombs == 0 {
        return current_element;
    }

    char::from_digit(neighbor_bombs, 10).unwrap()
}
