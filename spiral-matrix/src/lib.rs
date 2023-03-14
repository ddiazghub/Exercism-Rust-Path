pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut matrix = vec![vec![0; size]; size];
    let mut acc = 1;

    for i in 0..size {
        for j in i..size - i {
            matrix[i][j] = acc;
            acc += 1;
        }

        for j in i + 1..size - i {
            matrix[j][size - (i + 1)] = acc;
            acc += 1;
        }

        for j in (i..size - (i + 1)).rev() {
            matrix[size - (i + 1)][j] = acc;
            acc += 1;
        }

        for j in (i + 1..size - (i + 1)).rev() {
            matrix[j][i] = acc;
            acc += 1;
        }
    }

    matrix
}
