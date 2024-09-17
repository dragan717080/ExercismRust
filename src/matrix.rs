fn main() {
    let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
    assert_eq!(matrix.column(3), Some(vec![3, 6, 9]));
}

pub struct Matrix {
    a: Vec<Vec<u32>>
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let a: Vec<Vec<u32>> = input
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|row| 
                row.split(" ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>()
            )
            .collect();

        Self { a }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.a.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let rows = self.a.len();

        Some((0..rows).map(|j| self.a[j].get(col_no - 1).cloned()).collect::<Option<Vec<u32>>>()?)
    }
}
