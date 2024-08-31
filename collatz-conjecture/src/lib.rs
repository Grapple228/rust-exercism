pub fn collatz(mut n: u64) -> Option<u64> {
    for i in 0.. {
        match n {
            0 => break,
            1 => return Some(i),
            _ => n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 },
        }
    }

    None
}
