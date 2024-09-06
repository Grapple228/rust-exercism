use std::collections::HashMap;

fn is_valid(nucleotide: &char) -> bool {
    matches!(nucleotide, 'A' | 'C' | 'G' | 'T')
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(&nucleotide) {
        return Err(nucleotide);
    }

    if let Some(c) = dna.chars().find(|c| !is_valid(c)) {
        return Err(c);
    }

    Ok(dna.chars().filter(|n| *n == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();

    for nucleotide in "ACGT".chars() {
        counts.insert(nucleotide, count(nucleotide, dna)?);
    }

    Ok(counts)
}
