pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut n = n;
    let mut i = 2;
    while n > 1 {
        if n % i == 0 {
            result.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    result
}
