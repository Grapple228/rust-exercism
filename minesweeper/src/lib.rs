const MINE: u8 = b'*';

fn update_square(minefield: &mut [Vec<u8>], row: usize, col: usize) {
    for r in row.saturating_sub(1)..=(row + 1).min(minefield.len() - 1) {
        for c in col.saturating_sub(1)..=(col + 1).min(minefield[0].len() - 1) {
            if minefield[r][c] == MINE {
                continue;
            }

            if minefield[r][c].is_ascii_digit() {
                minefield[r][c] += 1;
            } else {
                minefield[r][c] = b'1';
            }
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<Vec<u8>> = minefield.iter().map(|s| s.as_bytes().to_vec()).collect();

    let row_count = result.len();
    let col_count = if row_count > 0 { result[0].len() } else { 0 };

    for row in 0..row_count {
        for col in 0..col_count {
            if result[row][col] != MINE {
                continue;
            }

            update_square(&mut result, row, col);
        }
    }

    result
        .into_iter()
        .map(|row| unsafe { String::from_utf8_unchecked(row) })
        .collect()
}
