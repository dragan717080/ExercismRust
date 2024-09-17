use std::collections::BTreeMap;

fn main() {
    let input = [
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ].iter().cloned().collect::<BTreeMap<_, _>>();

    print!("{:?}", transform(&input));
}

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().flat_map(|(&n, vec)| vec.iter().map(move |c| (c.to_ascii_lowercase(), n))).collect()
}
