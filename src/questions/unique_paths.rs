use crate::*;

test! {
    test_1: unique_paths(3, 7), 28,
    test_2: unique_paths(3, 2), 3,
    test_3: unique_paths(3, 3), 6,
}

pub fn unique_paths(m: usize, n: usize) -> usize {
    let mut matrix = vec![vec![1; n]; m];

    for i in 0..m {
        for j in 0..n {
            if i > 0 && j > 0 {
                matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1];
            }
        }
    }

    matrix[m - 1][n - 1]
}
