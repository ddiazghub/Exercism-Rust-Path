// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map: HashMap<&str, u32> = magazine.into_iter()
        .fold(HashMap::new(), |mut map, &word| {
            match map.get_mut(word) {
                Some(count) => *count += 1,
                None => drop(map.insert(word, 1))
            };

            map
        });

    for &word in note {
        match magazine_map.get_mut(word) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return false
        }
    }

    true
}
