fn main() {
    println!("{}", sing(3, 0));
}

pub fn verse(n: u32) -> String {
    if n == 1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\n".to_owned() +
            "Take it down and pass it around, no more bottles of beer on the wall.\n";
    }

    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\n".to_owned() +
            "Go to the store and buy some more, 99 bottles of beer on the wall.\n";
    }

    let first_sentence = format!("{} bottles of beer on the wall, {} bottles of beer.\n", n, n);


    let s_at_end = if n - 1 != 1 { "s" } else { "" };
    let quantity = if n != 1 { (n - 1).to_string() } else { "no more".to_string() };
    let second_sentence = format!("Take one down and pass it around, {} bottle{} of beer on the wall.\n", quantity, s_at_end);

    first_sentence + &second_sentence
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();

    for n in (end..start + 1).rev() {
        s += &verse(n);
        // Formatting
        if n != end {
            s += "\n";
        }
    }

    s
}
