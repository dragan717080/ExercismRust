fn main() {
    println!("{:?}", collatz(1000000));
}

pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }

    let mut steps = 0;
    let mut current_result = n;

    while current_result > 1 {
        if current_result % 2 == 0 {
            current_result /= 2;
        } else {
            current_result = current_result * 3 + 1;
        }

        steps += 1;
    }

    Some(steps)
}
