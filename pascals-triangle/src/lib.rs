pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();

        for i in 0..row_count {
            let mut row = Vec::new();

            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    row.push(
                        rows[i as usize - 1][j as usize - 1] + rows[i as usize - 1][j as usize],
                    );
                }
            }

            rows.push(row);
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
