use rand::{thread_rng, Rng};

fn main() {
    let p: u64 = 23;
    let g: u64 = 5;
    let a = private_key(p);
    let b_pub = public_key(p, g, a);
    let secret = secret(p, b_pub, a);

    println!("{}", a);
    println!("{}", b_pub);
    println!("{}", secret);
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}
pub fn modular_exponentiation(num: u64, power: u64, modulus: u64) -> u64 {
    let (mut power, mut num, modulus, mut result) =
        (power, num as u128, modulus as u128, 1 as u128);
    while power > 0 {
        if power % 2 == 1 {
            result = (result * num) % modulus;
        }
        num = (num * num) % modulus;
        power = power / 2
    }
    result as u64
}
