fn main() {
    let s1 = "GAGCCTACTAACGGGAT";
    let s2 = "CATCGTAATGACGGCCT";

    println!("{:?}", hamming_distance(s1, s2));
}

pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut result = 0 as usize;

    let s2_chars = s2.chars().collect::<Vec<char>>();

    for (i, c) in s1.char_indices() {
        if !['A', 'C', 'G', 'T'].contains(&c) || !['A', 'C', 'G', 'T'].contains(&s2_chars[i]) {
            return None;
        }

        if !(s2_chars[i] == c) {
            result += 1;
        }
    }

    Some(result)
}
