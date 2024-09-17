fn main() {
    println!("{}", square(32));
}

pub fn square(s: u32) -> u64 {
    if s < 1_u32 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..65).into_iter().fold(0, |acc, x| acc + square(x))
}
