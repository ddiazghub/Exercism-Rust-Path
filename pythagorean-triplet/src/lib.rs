use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triples = HashSet::new();

    for a in 1..sum / 3 {
        let bc = sum - a;

        for b in a + 1..(bc + 1) / 2 {
            let c = bc - b;

            if a*a + b*b == c*c {
                triples.insert([a, b, c]);
            }
        }
    }

    triples
}
