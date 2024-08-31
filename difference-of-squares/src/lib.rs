type Output = u64;

pub fn square_of_sum(n: Output) -> Output {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: Output) -> Output {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: Output) -> Output {
    square_of_sum(n) - sum_of_squares(n)
}
