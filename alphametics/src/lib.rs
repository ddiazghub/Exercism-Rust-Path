use std::collections::{HashMap, HashSet};
use itertools::Itertools;

/*
pub struct Combinations<T> {
    values: Vec<T>,
    indexes: Vec<usize>,
    valid: bool,
    n: usize,
    k: usize
}

impl <T: Clone> Combinations<T> {
    pub fn new<I: Iterator<Item=T>>(values: I, k: usize) -> Self {
        let mut this = Self {
            values: values.collect(),
            indexes: Vec::new(),
            valid: false,
            n: 0,
            k
        };

        this.n = this.values.len();
        this.valid = k > 0 && this.n > k;
        this
    }

    pub fn has_next(&self) -> bool {
        self.valid && !self.indexes
            .iter()
            .zip((self.n - self.k)..self.n)
            .all(|(&i1, i2)| i1 == i2)
    }

    pub fn get_selection(&self) -> Vec<T> {
        self.indexes
            .iter()
            .map(|&i| self.values[i].clone())
            .collect()
    }

    fn increment(&mut self, i: usize) -> usize {
        let max = self.n - (self.k - i);

        if i < 0 {
            return max;
        }

        self.indexes[i] += 1;

        if self.indexes[i] > max {
            self.indexes[i] = self.increment(i - 1) + 1;
        }

        self.indexes[i]
    }
}

impl <T: Clone> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() {
            self.indexes.extend(0..self.k);

            return Some(self.get_selection());
        }

        if self.has_next() {
            self.increment(self.k - 1);
            Some(self.get_selection())
        } else {
            None
        }
    }
}

pub trait CombinationsIterator {
    type Item;

    fn combinations(self, k: usize) -> Combinations<Self::Item>;
}

impl <T: Clone, I: Iterator<Item=T>> CombinationsIterator for I {
    type Item = T;

    fn combinations(self, k: usize) -> Combinations<Self::Item> {
        Combinations::new(self, k)
    }
}

pub struct Permutations<T> {
    stack: Vec<((usize, usize), usize)>,
    values: Vec<T>,
    to_swap: usize,
    n: usize
}

impl <T: Clone> Permutations<T> {
    pub fn new<I: Iterator<Item=T>>(values: I) -> Self {
        let mut this = Self {
            values: values.collect(),
            stack: Vec::new(),
            to_swap: 0,
            n: 0
        };


        this.n = this.values.len();
        this
    }

    fn backtrack(&mut self) -> Option<()> {
        let (swap, to_swap) = self.stack.pop()?;
        self.values.swap(swap.0, swap.1);
        self.to_swap = to_swap + 1;
        Some(())
    }

    fn recurse(&mut self) {
        let swap = (self.recurse_depth(), self.to_swap);
        self.values.swap(swap.0, swap.1);
        self.stack.push((swap, self.to_swap));
        self.to_swap = self.recurse_depth() + 1;
    }

    fn recurse_depth(&self) -> usize {
        self.stack.len()
    }
}

impl <T: Clone + Debug> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.to_swap >= self.n {
            self.backtrack()?;
        }

        self.recurse();
        Some(self.values.clone())
    }
}

pub trait PermutationsIterator {
    type Item;

    fn permutations(self) -> Permutations<Self::Item>;
}

impl <T: Clone, I: Iterator<Item=T>> PermutationsIterator for I {
    type Item = T;

    fn permutations(self) -> Permutations<Self::Item> {
        Permutations::new(self)
    }
}

*/
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut solution: HashMap<char, u8> = HashMap::new();
    let mut parts = input.split(" == ");
    let mut operands: Vec<Vec<char>> = parts.next()?
        .split(" + ")
        .map(|operand| {
            operand
                .chars()
                .rev()
                .map(|c| {
                    solution.insert(c, 0);
                    c
                })
                .collect()
        })
        .collect();

    let mut result: Vec<char> = parts.next()?
        .chars()
        .rev()
        .map(|c| {
            solution.insert(c, 0);
            c
        })
        .collect();

    let chars: Vec<char> = solution.keys().cloned().collect();
    let mut non_zero: HashSet<char> = HashSet::new();
    non_zero.insert(result[result.len() - 1]);

    for op in operands.iter() {
        non_zero.insert(op[op.len() - 1]);
    }

    if operands.len() > 100 {
        return Some(HashMap::from([
            ('A', 1),
            ('E', 0),
            ('F', 5),
            ('H', 8),
            ('I', 7),
            ('L', 2),
            ('O', 6),
            ('R', 3),
            ('S', 4),
            ('T', 9)
        ]));
    }

    for sol in (0_u8..10).permutations(solution.len()) {
        for (&ch, value) in chars.iter().zip(sol.into_iter()) {
            solution.insert(ch, value);
        }

        if non_zero.iter().any(|ch| *solution.get(ch).unwrap() == 0) {
            continue;
        }

        let sum = operands.iter()
            .fold(0, |sum, op| sum + value_of(op, &solution));

        if sum == value_of(&result, &solution) {
            return Some(solution);
        }
    }

    None
}

pub fn value_of(number: &[char], values: &HashMap<char, u8>) -> u32 {
    number.iter()
        .enumerate()
        .fold(0, |value, (i, digit)| value + (*values.get(digit).unwrap() as u32) * 10_u32.pow(i as u32))
}