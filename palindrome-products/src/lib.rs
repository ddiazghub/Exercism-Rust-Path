use std::collections::BTreeSet;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let s = value.to_string();
        let half = s.chars().count() / 2;

        let is_palindrome = s.chars()
            .zip(s.chars().rev())
            .take(half)
            .all(|(ch1, ch2)| ch1 == ch2);

        is_palindrome.then_some(Self(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut products: BTreeSet<_> = (min..=max)
        .flat_map(|i| (i..=max).map(move |j| i * j))
        .collect();

    let first = products.iter()
        .copied()
        .find_map(Palindrome::new);

    let last = products.into_iter()
        .rev()
        .find_map(Palindrome::new);

    first.zip(last)
}
