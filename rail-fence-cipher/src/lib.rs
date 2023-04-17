use std::{iter::{Chain, Cycle, Rev, Zip}, ops::Range};

type ZigZagIter<I> = Zip<Cycle<Chain<Range<usize>, Rev<Range<usize>>>>, I>;
pub struct ZigZag<I: Iterator>(ZigZagIter<I>);

impl<I: Iterator> ZigZag<I> {
    pub fn new(iter: I, range: Range<usize>) -> Self {
        Self(
            (range.start..range.end - 1)
                .chain((range.start + 1..range.end).rev())
                .cycle()
                .zip(iter)
        )
    }
}

impl<I: Iterator> Iterator for ZigZag<I> {
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub trait ZigZagIterator {
    type Iter;

    fn zigzag(self, range: Range<usize>) -> ZigZag<Self::Iter> where Self::Iter: Iterator;
}

impl<T: Iterator> ZigZagIterator for T {
    type Iter = Self;

    fn zigzag(self, range: Range<usize>) -> ZigZag<Self::Iter> {
        ZigZag::new(self, range)
    }
}

pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails as usize)
    }

    pub fn encode(&self, text: &str) -> String {
        text
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .zigzag(0..self.0)
            .fold(vec![String::new(); self.0], |mut rails, (rail, ch)| {
                rails[rail].push(ch);
                rails
            })
            .into_iter()
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rail_indexes: Vec<_> = cipher
            .char_indices()
            .zigzag(0..self.0)
            .map(|(rail, (i, _))| (rail, i))
            .collect();
        
        rail_indexes.sort_unstable();

        let mut chars: Vec<_> = rail_indexes
            .into_iter()
            .zip(cipher.chars())
            .map(|((_, i), ch)| (i, ch))
            .collect();

        chars.sort_unstable();

        chars
            .into_iter()
            .map(|(_, ch)| ch)
            .collect()
    }
}