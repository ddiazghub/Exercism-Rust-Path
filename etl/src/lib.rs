use std::{collections::BTreeMap};

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter()
        .flat_map(|(points, letters)| {
            letters.into_iter()
                .map(|letter| (letter.to_ascii_lowercase(), *points))
        })
        .collect()
}
