use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let chars = char_count_map(&word.to_lowercase());
    let anagrams: HashSet<&str> = HashSet::with_capacity(possible_anagrams.len());

    possible_anagrams
        .iter()
        .fold( anagrams, |mut anagrams, possible| {
            let p = possible.to_lowercase();

            if !(p == word.to_lowercase() || p.len() != word.len()) {
                let char_counts = char_count_map(&p);

                let is_anagram = char_counts
                    .iter()
                    .all(|(ch, &frequency)| {
                        match chars.get(ch) {
                            Some(&freq2) => freq2 == frequency,
                            _ => false
                        }
                    });

                if is_anagram {
                    anagrams.insert(*possible);
                }
            }

            anagrams
    })
}

fn char_count_map(word: &str) -> HashMap<char, i32> {
    word
        .chars()
        .into_iter()
        .fold(HashMap::with_capacity(word.len()), |mut word_chars, ch| {
            match word_chars.get_mut(&ch) {
                Some(v) => *v += 1,
                None => drop(word_chars.insert(ch, 0))
            };

            word_chars
        })
}
