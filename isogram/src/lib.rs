pub fn hash_check(candidate: &str) -> bool {
    let mut hs = std::collections::HashSet::new();

    candidate
        .to_lowercase()
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| hs.insert(c))
}

pub fn check(candidate: &str) -> bool {
    const A_LCASE: u8 = b'a';

    candidate
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}
