fn main() {
    let words = ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];

    println!("{}", build_proverb(&words));
}

pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();

    if list.len() == 0 {
        return result;
    }

    for (i, word) in list[1..list.len()].iter().enumerate() {
        println!("I: {}", i);
        result += &format!("For want of a {} the {} was lost.\n", list[i], word);
    }

    result += &format!("And all for the want of a {}.", list[0]);

    result
}
