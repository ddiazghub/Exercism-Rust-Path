use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    for &factor in factors.iter() {
        if factor > 0 {
            multiples.extend((factor..limit).step_by(factor as usize));
        }
    }

    multiples
        .into_iter()
        .sum()
}
