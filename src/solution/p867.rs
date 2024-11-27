pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut new_matrix = vec![vec![0;rows];cols];
        for i in 0..rows {
            for j in 0..cols {
                new_matrix[j][i] = matrix[i][j];
            }
        }
        new_matrix
    }
}