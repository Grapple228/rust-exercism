use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();

    for factor in factors {
        if *factor == 0 {
            continue;
        }

        for i in (1..limit).filter(|i| i % factor == 0) {
            set.insert(i);
        }
    }

    set.iter().sum()
}
