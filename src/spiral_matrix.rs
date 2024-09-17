fn main() {
    print!("{:?}", spiral_matrix(7));
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut a = vec![vec![0; size as usize]; size as usize];

    if size == 0 {
        return a;
    }

    let steps = size * 2 - 1;
    let insert_count_each_step = get_insert_count_each_step(size);

    for i in 0..steps {
        let step = i as i32 % 4;

        match step {
            0 => left_to_right_on_top(&mut a, i, &insert_count_each_step),
            1 => top_to_bottom_on_right(&mut a, i, &insert_count_each_step),
            2 => right_to_left_on_bottom(&mut a, i, &insert_count_each_step),
            3 => bottom_to_top_on_left(&mut a, i, &insert_count_each_step),
            _ => ()
        }
    }

    a
}

fn left_to_right_on_top(a: &mut Vec<Vec<u32>>, current_step: u32, insert_count_each_step: &Vec<u32>) {
    // Skip on each side equal to current row
    let row = current_step / 4;

    let insert_count = insert_count_each_step[current_step as usize];

    let previous_elements = a.into_iter().flat_map(|v| v).filter(|d| **d != 0).count() as u32;
    let end_index = a.len() as u32 - 1_u32 - row;
    let start_index = end_index + 1 - insert_count;

    let element_count = end_index - start_index;

    let elements = (previous_elements..previous_elements + element_count + 1).collect::<Vec<u32>>();

    // This variable is needed when left to right, since initially previous_elements will be 0
    let to_skip = if row == 0 { 1 } else { 0 };

    for j in start_index..=end_index {
        a[row as usize][j as usize] = elements[(j - start_index) as usize] + to_skip;
    }
}

fn top_to_bottom_on_right(a: &mut Vec<Vec<u32>>, current_step: u32, insert_count_each_step: &Vec<u32>) {
    let col = a.len() as u32 - current_step / 4 - 1;

    let skip_top_count = current_step / 4;
    let insert_count = insert_count_each_step[current_step as usize];

    let previous_elements = a.into_iter().flat_map(|v| v).filter(|d| **d != 0).count() as u32;

    let end_index = a.len() as u32 - skip_top_count - 1;
    let start_index = end_index + 1 - insert_count;

    let elements = (previous_elements..previous_elements + end_index + 1).collect::<Vec<u32>>();

    for i in start_index..=end_index {
        a[i as usize][col as usize] = elements[(i - start_index) as usize];
    }
}

fn right_to_left_on_bottom(a: &mut Vec<Vec<u32>>, current_step: u32, insert_count_each_step: &Vec<u32>) {
    let row = a.len() as u32 - current_step / 4 - 1;

    let skip_right_count = current_step / 4;

    let insert_count = insert_count_each_step[current_step as usize];

    let previous_elements = a.into_iter().flat_map(|v| v).filter(|d| **d != 0).count() as u32;

    let end_index = a.len() as u32 - 1 - skip_right_count;
    let start_index = end_index + 1 - insert_count;
    let element_count = end_index - start_index;

    let elements = (previous_elements..previous_elements + element_count + 1).rev().collect::<Vec<u32>>();

    for j in start_index..=end_index {
        a[row as usize][j as usize] = elements[(j - start_index) as usize];
    }
}

fn bottom_to_top_on_left(a: &mut Vec<Vec<u32>>, current_step: u32, insert_count_each_step: &Vec<u32>) {
    let col = current_step / 4;

    let skip_bottom_count = current_step / 4;

    let insert_count = insert_count_each_step[current_step as usize];

    let previous_elements = a.into_iter().flat_map(|v| v).filter(|d| **d != 0).count() as u32;

    let end_index = a.len() as u32 - skip_bottom_count - 1;
    let start_index = end_index + 1 - insert_count;

    let element_count = end_index - start_index;

    let elements = (previous_elements..previous_elements + element_count + 1).rev().collect::<Vec<u32>>();

    for i in start_index..=end_index {
        a[i as usize][col as usize] = elements[(i - start_index) as usize];
    }
}

fn get_insert_count_each_step(size: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let steps = size * 2 - 1;

    let mut current = size;
    while result.len() < steps as usize {
        if current == size {
            result.extend(&vec![current, current, current])
        } else {
            result.extend(&vec![current, current]) 
        }

        current -= 1;
    }

    result
}
