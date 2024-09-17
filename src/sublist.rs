fn main() {
    let a = [0, 1, 2, 3, 4, 5];
    let b = [0, 1, 2];

    println!("{:?}", sublist(&a, &b));
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison
where
    T: PartialEq,
{
    if a.len() == 0 {
        if b.len() > 0 {
            return Comparison::Sublist
        } else {
            return Comparison::Equal;
        }
    }

    if b.len() == 0 {
        return Comparison::Superlist;
    }

    // A is maybe superlist of B
    if a.len() > b.len() {
        // Find the indexes of b[0] in a
        let indexes_of_b_first_in_a: Vec<usize> = a.iter().enumerate().filter_map(|(i, d)| if *d == b[0] { Some(i) } else { None }).collect();
        if indexes_of_b_first_in_a.len() == 0 {
            return Comparison::Unequal;
        }

        for index in indexes_of_b_first_in_a {
            if a[index..index + b.len()] == *b {
                return Comparison::Superlist;
            }
        }

        return Comparison::Unequal;
    }

    // B is maybe superlist of A
    if a.len() < b.len() {
        // Find the indexes of a[0] in b
        let indexes_of_a_first_in_b: Vec<usize> = b.iter().enumerate().filter_map(|(i, d)| if *d == a[0] { Some(i) } else { None }).collect();
        if indexes_of_a_first_in_b.len() == 0 {
            return Comparison::Unequal;
        }

        for index in indexes_of_a_first_in_b {
            if b[index..index + a.len()] == *a {
                return Comparison::Sublist;
            }
        }

        return Comparison::Unequal;
    }

    // They are maybe equal
    if a == b { Comparison::Equal } else { Comparison::Unequal }
}
