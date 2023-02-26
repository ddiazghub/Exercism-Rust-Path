use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub struct Slice<'a, T: PartialEq>(&'a [T]);

impl <'a, T: PartialEq> Slice<'a, T> {
    pub fn is_sublist_of(&self, other: &Self) -> bool {
        let (length_sub, length_super) = (self.0.len(), other.0.len());

        length_sub == 0 || (0..(length_super - length_sub + 1))
            .any(|i| {
                other.0[i] == self.0[0] &&
                    Slice(&other.0[i..(i + self.0.len())]) == Slice(&self.0)
            })
    }
}

impl <'a, T: PartialEq> PartialEq for Slice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        let (first, second)  = (self.0, other.0);

        first.len() == second.len() && first
            .iter()
            .zip(second.iter())
            .all(|(element1, element2)| *element1 == *element2)
    }
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match first.len().cmp(&second.len()) {
        Ordering::Equal if Slice(first) == Slice(second) => Comparison::Equal,
        Ordering::Greater if Slice(second).is_sublist_of(&Slice(first)) => Comparison::Superlist,
        Ordering::Less if Slice(first).is_sublist_of(&Slice(second)) => Comparison::Sublist,
        _ => Comparison::Unequal
    }
}