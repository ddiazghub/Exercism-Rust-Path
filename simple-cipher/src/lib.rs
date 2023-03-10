use std::sync::mpsc::channel;
use rand;
use rand::distributions::Uniform;
use rand::Rng;

const A: u8 = 'a' as u8;
const ALPHABET_SIZE: u8 = 26;

trait StrZipMap {
    fn zip_map<F: Fn((char, char)) -> Option<char>>(&self, other: &Self, mapper: F) -> Option<String>;
}

impl StrZipMap for str {
    fn zip_map<F: Fn((char, char)) -> Option<char>>(&self, other: &Self, mapper: F) -> Option<String> {
        self.chars()
            .zip(other.chars())
            .map(mapper)
            .collect()
    }
}

fn encode_char(key: char, ch: char) -> Option<char> {
    match key {
        'a'..='z' => Some({
            let sh = key as u8 - A;
            let shifted = ((ch.to_ascii_lowercase() as u8 - A + sh) % ALPHABET_SIZE) + A;
            shifted as char
        }),
        _ => None
    }
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.len() == 0 {
        None
    } else {
        let mut key = key.to_string();

        while key.len() < s.len() {
            key.push('a');
        }

        key.zip_map(s, |(k, ch)| encode_char(k, ch))
    }
}

fn decode_char(key: char, ch: char) -> Option<char> {
    match key {
        'a'..='z' => Some({
            let sh = key as u8 - A;
            let shifted = ((ch.to_ascii_lowercase() as u8 - A + ALPHABET_SIZE - sh) % ALPHABET_SIZE) + A;
            shifted as char
        }),
        _ => None
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.len() == 0 {
        None
    } else {
        let mut key = key.to_string();

        while key.len() < s.len() {
            key.push('a');
        }

        key.zip_map(s, |(k, ch)| decode_char(k, ch))
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = rand::thread_rng()
        .sample_iter(Uniform::from(0..26))
        .take(100)
        .map(|n| (n + A) as char)
        .collect();

    let ciphertext = encode(&key, s).unwrap();
    (key, ciphertext)
}
