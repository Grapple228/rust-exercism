const ZERO: u32 = b'0' as u32;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");

    if isbn.len() != 10 {
        return false;
    }

    isbn.char_indices()
        .map(|(i, c)| match c {
            'X' if i == 9 => 10,
            other => (other as u32 - ZERO) * (10 - i as u32),
        })
        .sum::<u32>()
        % 11
        == 0
}
