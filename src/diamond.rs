fn main() {
    println!("{:?}", get_diamond('D'));
}

pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();

    let letter_ord = c as u32 - 65;

    let total_steps = 2 * letter_ord + 1;
    let mut spaces_around = total_steps as usize / 2;
    let mut spaces_inside: usize;

    for i in 0..letter_ord + 1 {
        spaces_inside = (2*i).checked_sub(1).unwrap_or(0) as usize;

        let row = get_row(i, spaces_around, spaces_inside);

        spaces_around = spaces_around.checked_sub(1).unwrap_or(0);

        result.push(row);
    }

    for i in (0..letter_ord).rev() {
        spaces_around += 1;
        spaces_inside = (2*i).checked_sub(1).unwrap_or(0) as usize;

        let row = get_row(i, spaces_around, spaces_inside);

        result.push(row);
    }

    result
}

fn get_row(i: u32, spaces_around: usize, spaces_inside: usize) -> String {
    let letter = &char::from_u32(65 + i).unwrap().to_string();
    let second_letter = if i != 0 { letter } else { "" };

    let row = format!(
        "{}{}{}{}{}",
        " ".repeat(spaces_around),
        letter,
        " ".repeat(spaces_inside),
        second_letter,
        " ".repeat(spaces_around)
    );

    row
}
