use std::collections::HashMap;
use std::str::Chars;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn is_valid_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'A'|'C'|'G'|'T' => true,
        _ => false
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if is_valid_nucleotide(nucleotide) {
        dna.chars()
            .try_fold(0, |c, n| match (is_valid_nucleotide(n), n == nucleotide) {
                (true, true) => Ok(c + 1),
                (true, false) => Ok(c),
                _ => Err(n)
            })
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<_, usize> = NUCLEOTIDES.iter()
        .map(|&n| (n, 0))
        .collect();

    dna.chars()
        .try_fold(counts, |mut counts, nucleotide| {
            if is_valid_nucleotide(nucleotide) {
                *counts.entry(nucleotide).or_insert(0) += 1;
                Ok(counts)
            } else {
                Err(nucleotide)
            }
        })
}
