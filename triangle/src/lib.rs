use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

pub struct Triangle<T>([T; 3]);

impl <T: Copy + Clone + PartialEq+ PartialOrd + Add<Output=T> + From<u8>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Self> {
        Self::is_valid(&sides)
            .then_some(Self(sides))
    }

    pub fn is_valid(sides: &[T; 3]) -> bool {
        sides.iter().all(|&side| side > T::from(0)) &&
            sides[0] < sides[1] + sides[2] &&
            sides[1] < sides[0] + sides[2] &&
            sides[2] < sides[0] + sides[1]
    }

    pub fn is_equilateral(&self) -> bool {
        self.0[0] == self.0[1] && self.0[0] == self.0[2] && self.0[1] == self.0[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.0[0] != self.0[1] && self.0[0] != self.0[2] && self.0[1] != self.0[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.0[0] == self.0[1] || self.0[0] == self.0[2] || self.0[1] == self.0[2]
    }
}
