use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn count(lines: &[&str]) -> u32 {
    let mut count = 0;
    let mut corners: HashSet<usize> = HashSet::new();
    let mut corner_pairs: HashMap<[usize; 2], u32> = HashMap::new();
    
    for line in lines {
        let mut row_corners: Vec<usize> = Vec::new();
        let mut inside = false;

        for (i, ch) in line.chars().enumerate() {
            match ch {
                '+' => {
                    for &corner in row_corners.iter() {
                        *corner_pairs.entry([corner, i]).or_insert(0) += 1;
                    }
    
                    row_corners.push(i);
                    corners.insert(i);
                    inside = true;
                },
                ch if ch != '|' => {
                    if corners.contains(&i) {
                        let to_delete: Vec<_> = corner_pairs
                            .iter()
                            .filter(|(pair, _)| pair[0] == i || pair[1] == i)
                            .map(|(&pair, _)| pair)
                            .collect();

                        for pair in to_delete {
                            count += corner_pairs.remove(&pair).unwrap().combs(2);
                        }

                        corners.clear();

                        corners.extend(
                            corner_pairs
                                .iter()
                                .flat_map(|(&pair, _)| pair)
                        );
                    }

                    if inside && ch != '-' {
                        row_corners.clear();
                        inside = false;
                    }
                }
                _ => {}
            }
        }
    }

    count + corner_pairs
        .into_values()
        .map(|count| count.combs(2))
        .sum::<u32>()
}

pub trait Combs {
    fn combs(self, r: Self) -> Self;
    fn fact(self) -> Self;
}

impl Combs for u32 {
    fn combs(self, r: Self) -> Self {
        match self.cmp(&r) {
            Ordering::Equal => 1,
            Ordering::Less => 0,
            _ => self.fact() / ((self - r).fact() * r.fact())
        }
    }

    fn fact(self) -> Self {
        (1..self + 1).product()
    }
}