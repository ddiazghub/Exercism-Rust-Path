use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let result = candidate.chars()
        .filter(|&ch| ch != ' ' && ch != '-')
        .try_fold(HashSet::new(), |mut found, ch| {
            found.insert(ch.to_ascii_lowercase())
                .then_some(found)
        });

    result.is_some()
}
