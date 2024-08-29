/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() <= 1 {
        return false;
    }

    let mut sum = 0;

    for (i, c) in code.chars().rev().enumerate() {
        if !c.is_ascii_digit() {
            return false;
        }

        let digit = c.to_digit(10).unwrap();

        if i % 2 == 0 {
            sum += digit;
        } else {
            let mut doubled = digit * 2;

            if doubled > 9 {
                doubled -= 9;
            }

            sum += doubled;
        }
    }

    sum % 10 == 0
}
