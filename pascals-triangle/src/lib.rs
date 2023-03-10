pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();

        for row in 0..row_count {
            let mut r = Vec::new();

            for i in 0..row + 1 {
                if i == 0 || i == row {
                    r.push(1);
                } else {
                    let row_value = rows[(row - 1) as usize][(i - 1) as usize] + rows[(row - 1) as usize][i as usize];
                    r.push(row_value);
                }
            }

            rows.push(r)
        }

        Self(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
