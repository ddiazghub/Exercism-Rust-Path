use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

pub fn private_key(p: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    hasher.finish().clamp(2, p - 1)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}

pub fn mod_pow(n: u64, pow: u64, modulo: u64) -> u64 {
    let decomposed: Vec<u64> = binary_decompose(pow);
    let powers_of_2: Vec<u64> = binary_decompose((decomposed.last().unwrap() << 1) - 1);

    let decomposed: HashSet<u64> = decomposed
        .into_iter()
        .collect();

    let n_mod = (n % modulo) as u128;

    let initial = match pow % 2 {
        0 => (1, n_mod),
        _ => (n_mod, n_mod)
    };

    let (m_pow, _) = &powers_of_2[1..]
        .iter()
        .fold(initial, |(mut m_pow, last_p), p2| {
            let sub_mod_pow = (last_p * last_p) % modulo as u128;

            if decomposed.contains(&p2) {
                m_pow *= sub_mod_pow;
            }

            (m_pow, sub_mod_pow)
        });

    (*m_pow % modulo as u128) as u64
}

pub fn binary_decompose(n: u64) -> Vec<u64> {
    let mut i = 1_u64;
    let mut values = Vec::new();

    while i <= n {
        if i & n > 0 {
            values.push(i);
        }

        i <<= 1;
    }

    values
}