use std::collections::HashMap;

const A_: u8 = 'a' as u8;
const Z_: u8 = 'z' as u8;
const A: u8 = 'A' as u8;

thread_local! {
    static SUBSTITUTIONS: HashMap<char, char> = (0..27)
        .into_iter()
        .flat_map(|i| [
            ((A_ + i) as char, (Z_ - i) as char),
            ((A + i) as char, (Z_ - i) as char) 
        ])
        .chain(
            ('0'..='9')
                .into_iter()
                .map(|digit| (digit, digit))
        )
        .collect::<HashMap<char, char>>();
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    SUBSTITUTIONS.with(|subs| plain
        .chars()
        .filter_map(|ch| subs.get(&ch))
        .enumerate()
        .map(|(i, &ch)| match i % 5 {
            0 if i > 0 => " ".to_string() + &ch.to_string(),
            _ => ch.to_string()
        })
        .collect()
    )
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    SUBSTITUTIONS.with(|subs| cipher
        .chars()
        .filter_map(|ch| subs.get(&ch))
        .collect()
    )
}
