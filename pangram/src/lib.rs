/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut result: u32 = 0;

    for c in sentence.chars() {
        if c.is_alphabetic() {
            result |= 1 << (c.to_ascii_lowercase() as u8 - b'a');
        }
    }

    result.count_ones() == 26
}
