use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub struct Factors {
    num: u64,
    current: u64,
    found: Vec<u64>,
    unloading: bool
}

impl Factors {
    pub fn new<T: Into<u64>>(num: T) -> Self {
        Self {
            num: num.into(),
            current: 0,
            unloading: false,
            found: Vec::new()
        }
    }
}

impl Iterator for Factors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.unloading, self.found.is_empty()) {
            (true, false) => self.found.pop(),
            (true, true) => None,
            _ => loop {
                self.current = self.current + 1;

                if Some(&self.current) == self.found.last() {
                    self.unloading = true;

                    break self.found.pop()
                }

                if self.num % self.current == 0 {
                    let div = self.num / self.current;

                    if div == self.current {
                        self.unloading = true;
                    } else {
                        self.found.push(div);
                    }

                    break Some(self.current)
                }
            }
        }
    }
}

trait FactorsIterator {
    fn factors(&self) -> Factors;
}

impl FactorsIterator for u64 {
    fn factors(&self) -> Factors {
        Factors::new(*self)
    }
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let aliquot: u64 = num.factors().sum();

    match aliquot.cmp(&(2*num)) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Deficient),
        _ => Some(Classification::Abundant)
    }
}
