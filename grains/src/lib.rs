use std::ops::RangeInclusive;

const ALLOWED_RANGE: RangeInclusive<u32> = 1..=64;

pub fn square(s: u32) -> u64 {
    assert!(
        ALLOWED_RANGE.contains(&s),
        "Square must me between {} and {}",
        ALLOWED_RANGE.start(),
        ALLOWED_RANGE.end()
    );

    1 << (s - 1)
}

pub fn total() -> u64 {
    ((1_u128 << 64) - 1) as u64
}
