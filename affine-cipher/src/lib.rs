use std::collections::HashSet;
use std::hash::Hash;

const A: i32 = 'a' as i32;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}


pub trait MMI: Sized + Eq + Hash {
    fn mmi(self, modulo: Self) -> Result<Self, AffineCipherError>;

    fn factors(&self) -> Vec<Self>;

    fn coprime_with(&self, other: Self) -> bool {
        let factors: HashSet<Self> = self.factors()
            .into_iter()
            .skip(1)
            .collect();

        !other.factors()
            .into_iter()
            .skip(1)
            .any(|factor| factors.contains(&factor))
    }
}

impl MMI for i32 {
    fn mmi(self, modulo: Self) -> Result<Self, AffineCipherError> {
        if self.coprime_with(modulo) {
            for i in 1.. {
                let target = modulo * i + 1;

                if target % self == 0 {
                    return Ok(target / self);
                }
            }
        }

        Err(AffineCipherError::NotCoprime(self))
    }

    fn factors(&self) -> Vec<Self> {
        match *self {
            0 => Vec::new(),
            _ => (1..=*self)
                .filter(|&factor| *self % factor == 0)
                .collect()
        }
        
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if a.coprime_with(26) {
        let ciphertext = plaintext
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .enumerate()
            .map(|(i, ch)| {
                let mut s = String::new();

                if i > 0 && i % 5 == 0 {
                    s.push(' ');
                }

                s.push(encode_char(ch, a, b));
                s
            })
            .collect();

        Ok(ciphertext)
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}

fn encode_char(ch: char, a: i32, b: i32) -> char {
    if ch.is_ascii_alphabetic() {
        let char_index = ch.to_ascii_lowercase() as i32 - A;
        (A + (a * char_index + b).rem_euclid(26)) as u8 as char
    } else {
        ch
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let a_mmi = a.mmi(26)?;

    let plaintext = ciphertext
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|ch| decode_char(ch, a_mmi, b))
        .collect();

    Ok(plaintext)
}

fn decode_char(ch: char, a_mmi: i32, b: i32) -> char {
    if ch.is_ascii_alphabetic() {
        let char_index = ch.to_ascii_lowercase() as i32 - A;
        (A + (a_mmi * (char_index - b)).rem_euclid(26)) as u8 as char
    } else {
        ch
    }
}