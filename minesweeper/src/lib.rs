pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();

    let width = if height > 0 {
        minefield[0].len()
    } else {
        0
    };

    let mut annotated_field: Vec<Vec<i32>> = vec![vec![0; width]; height];

    for (i, row) in minefield.iter().enumerate() {
        for (j, cell) in row.bytes().enumerate() {
            if cell as char == '*' {
                let (i2, j2) = (i as i64, j as i64);

                let (i_range, j_range) = (
                    (i2 - 1).clamp(0, height as i64 - 1)..=(i2 + 1).clamp(0, height as i64 - 1),
                    (j2 - 1).clamp(0, width as i64 - 1)..=(j2 + 1).clamp(0, width as i64 - 1)
                );

                annotated_field[i][j] = -1;

                for i2 in i_range {
                    for j2 in j_range.clone() {
                        if (i2 as usize, j2 as usize) != (i, j) && annotated_field[i2 as usize][j2 as usize] != -1 {
                            annotated_field[i2 as usize][j2 as usize] += 1;
                        }
                    }
                }
            }
        }
    }

    annotated_field.iter().map(|row| {
        row.iter().map(|cell| match cell {
            -1 => '*',
            0 => ' ',
            &n => char::from_digit(n as u32, 10).unwrap()
        }).collect::<String>()
    }).collect::<Vec<String>>()
}