fn main() {
    println!("{}", difference(10));
}

pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1..n + 1).sum::<u32>();

    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n + 1).fold(0, |acc, d| acc + d.pow(2))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
