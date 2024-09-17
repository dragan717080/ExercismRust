fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    fn square(n: i32) -> i32 {
        n.pow(2)
    }

    println!("{:?}", map(arr, square));
}

pub fn map<F, T, U>(input: Vec<T>, mut _function: F) -> Vec<U>
where F: FnMut(T) -> U {
    let mut v = Vec::with_capacity(input.len());

    for val in input {
        v.push(_function(val));
    }

    v
}
