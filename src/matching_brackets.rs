fn main() {
    println!("{}", brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
}

pub fn brackets_are_balanced(s: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();

    for c in s.chars() {
        if ['{', '[', '('].contains(&c) {
            brackets.push(c);
        } else if c == '}' {
            if brackets.pop() != Some('{') {
                return false;
            }
        } else if c == ']' {
            if brackets.pop() != Some('[') {
                return false;
            }
        } else if c == ')' {
            if brackets.pop() != Some('(') {
                return false;
            }
        }
    }

    brackets.len() == 0
}
