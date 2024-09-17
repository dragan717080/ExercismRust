fn main() {
    let n1 = 15;
    let n2 = 14;
    let n3 = 10;
    let n4 = 8;

    println!("{}", raindrops(n1));
    println!("{}", raindrops(n2));
    println!("{}", raindrops(n3));
    println!("{}", raindrops(n4));
}

pub fn raindrops(n: u32) -> String {
    let mut s = "".to_string();

    if n % 3 == 0 {
        s += "Pling";
    }

    if n % 5 == 0 {
        s += "Plang";
    }

    if n % 7 == 0 {
        s += "Plong";
    }

    if s.len() == 0 { n.to_string() } else { s }
}
