// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::fmt::Display;
use std::ops::{Add, Rem};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T, K> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: K
}

impl<T, K> Matcher<T, K> {
    pub fn new(_matcher: impl Fn(T) -> bool + 'static, _subs: K) -> Matcher<T, K> {
        Self {
            matcher: Box::new(_matcher),
            subs: _subs
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>(Vec<Matcher<T, &'static str>>);

impl<T: Clone + Display> Fizzy<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T, &'static str>) -> Self {
        self.0.push(_matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: IntoIterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String> {
        // unimplemented!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire

        _iter.into_iter()
            .map(move |element| {
                let string = self.0.iter()
                    .filter(|matcher| (*matcher.matcher)(element.clone()))
                    .map(|matcher| matcher.subs)
                    .collect::<String>();

                if string.is_empty() {
                    element.to_string()
                } else {
                    string
                }
            })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Clone + Add<Output=T> + Rem<Output=T> + PartialEq + From<u8> + Display>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|element| element % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|element| element % T::from(5) == T::from(0), "buzz"))
}
