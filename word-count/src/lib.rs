use std::collections::HashMap;

enum WordCharacterType {
    Alphanumeric,
    SingleApostrophe,
    Space,
    Invalid
}

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut w_count = HashMap::new();
    let mut word = String::new();
    let mut last = WordCharacterType::Space;
    println!("{words}");

    for ch in words.chars() {
        match ch {
            '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                word.push(ch.to_ascii_lowercase());
                last = WordCharacterType::Alphanumeric
            },
            '\'' => {
                match last {
                    WordCharacterType::Alphanumeric => word.push(ch),
                    _ => ()
                };

                last = WordCharacterType::SingleApostrophe
            },
            _ => {
                add_word(&mut w_count, &word, last);
                word.clear();
                last = WordCharacterType::Invalid;
            }
        }
    }

    add_word(&mut w_count, &word, last);
    println!("{w_count:?}");
    w_count
}

fn add_word(words: &mut HashMap<String, u32>, mut word: &str, last: WordCharacterType) {
    if word.len() > 0 {
        match last {
            WordCharacterType::SingleApostrophe => word = &word[..word.len() - 1],
            _ => ()
        };

        match words.get_mut(word) {
            Some(count) => *count += 1,
            None => drop(words.insert(word.to_string(), 1))
        };
    }
}
