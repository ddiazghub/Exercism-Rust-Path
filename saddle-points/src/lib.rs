use std::cmp::Ordering;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.len() > 0 && input[0].len() > 0 {
        let row_maximums: Vec<(u64, Vec<usize>)> = input.iter()
            .map(row_max)
            .collect();

        let column_minimums: Vec<(u64, Vec<usize>)> = (0..input[0].len())
            .into_iter()
            .map(|i: usize| column_min(input, i))
            .collect();

        let col_mins = &column_minimums;

        let saddle_points = row_maximums
            .into_iter()
            .enumerate()
            .map(|(row, (max, maximums)): (usize, (u64, Vec<usize>))| {
                maximums
                    .into_iter()
                    .filter(move |&column: &usize| col_mins[column].0 == max && input[row][column] == max)
                    .map(move |column: usize| (row, column))
            })
            .flatten()
            .collect();

        saddle_points
    } else {
        Vec::new()
    }
}

fn row_max(row: &Vec<u64>) -> (u64, Vec<usize>) {
    row.iter()
        .enumerate()
        .skip(1)
        .fold((row[0], vec![0]), find_max)
}

fn column_min(matrix: &[Vec<u64>], column: usize) -> (u64, Vec<usize>) {
    (1..matrix.len())
        .into_iter()
        .map(|i: usize| (i, matrix[i][column]))
        .fold((matrix[0][column], vec![0]), find_min)
}

fn find_max((max, mut positions): (u64, Vec<usize>), (i, &cell): (usize, &u64)) -> (u64, Vec<usize>) {
    match cell.cmp(&max) {
        Ordering::Greater => (cell, vec![i]),
        Ordering::Less => (max, positions),
        Ordering::Equal => {
            positions.push(i);
            (max, positions)
        }
    }
}

fn find_min((min, mut positions): (u64, Vec<usize>), (i, cell): (usize, u64)) -> (u64, Vec<usize>) {
    match cell.cmp(&min) {
        Ordering::Greater => (min, positions),
        Ordering::Less => (cell, vec![i]),
        Ordering::Equal => {
            positions.push(i);
            (min, positions)
        }
    }
}