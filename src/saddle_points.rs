fn main() {
    let input = &[vec![3, 1, 3], vec![3, 2, 4]];
    println!("{:?}", find_saddle_points(input));
}

pub fn find_saddle_points(a: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if a[0].len() == 0 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let rows = a.len();
    let cols = a[0].len();

    let row_maxes = a.iter().map(|row| *row.iter().max().unwrap()).collect::<Vec<u64>>();

    let transposed = transpose(a.to_vec());

    let col_mins = transposed.iter().map(|col| *col.iter().min().unwrap()).collect::<Vec<u64>>();

    for i in 0..rows {
        for j in 0..cols {
            if a[i][j] == row_maxes[i] && a[i][j] == col_mins[j] {
                result.push((i, j));
            }
        }
    }

    result
}

fn transpose<T: Clone + Default>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut res: Vec<Vec<T>> = vec![vec![Default::default(); rows]; cols];

    for i in 0..cols {
        for j in 0..rows {
            res[i][j] = matrix[j][i].clone();
        }
    }

    res
}
