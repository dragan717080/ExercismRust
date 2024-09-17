fn main() {
    let pt = PascalsTriangle::new(10);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
        vec![1, 6, 15, 20, 15, 6, 1],
        vec![1, 7, 21, 35, 35, 21, 7, 1],
        vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
        vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
    ];
    assert_eq!(pt.rows(), expected);
}

pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = Vec::new();

        for i in 0..self.row_count {
            let mut row: Vec<u32> = Vec::new();

            for j in 0..i + 1 {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    let element = result[(i - 1) as usize][(j - 1) as usize] + result[(i - 1) as usize][j as usize];
                    row.push(element);
                }
            }

            result.push(row);
        }

        result
    }
}
