use std::collections::BTreeMap;

#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self(scores.iter().map(u32::clone).collect())
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        if self.0.len() > 0 {
            Some(self.0[self.0.len() - 1])
        } else {
            None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().map(u32::clone)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let map = self.0.iter()
            .fold(BTreeMap::new(), |mut map, &score| {
                *map.entry(score).or_insert(0_u32) += 1;
                map
            });

        let mut top_three = Vec::new();

        for (&score, &freq) in map.iter().rev() {
            for i in 0..freq {
                if top_three.len() < 3 {
                    top_three.push(score);
                }
            }
        }

        top_three
    }
}
