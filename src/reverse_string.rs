extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "Some string with unicode Chars: udüu";

    println!("{}", reverse(s));
}

pub fn reverse(input: &str) -> String {
    let reverse: String = input
        .graphemes(true)
        .rev()
        .collect();
    return reverse
}
