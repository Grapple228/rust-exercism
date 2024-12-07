pub fn encode(source: &str) -> String {
    let mut result = String::new();

    let mut chars = source.chars().peekable();

    let mut count = 0;

    while let Some(c) = chars.next() {
        count += 1;

        if chars.peek() != Some(&c) {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push(c);

            count = 0;
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();

    let mut number_str = String::new();

    for next in source.chars() {
        if next.is_ascii_digit() {
            number_str.push(next);
        } else {
            let count = number_str.parse::<usize>().unwrap_or(1);

            result.push_str(&next.to_string().repeat(count));

            number_str.clear();
        }
    }

    result
}
