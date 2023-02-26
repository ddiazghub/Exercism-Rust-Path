use std::fmt::{Debug, Formatter};

const DEFAULT_CAPACITY: usize = 97;

#[derive(PartialEq, Eq, Clone)]
pub struct CustomSet<T> {
    elements: Vec<Vec<T>>,
    size: usize,
    capacity: usize
}

impl <T: Debug + Clone> Debug for CustomSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let elements: Vec<T> = self.elements
            .iter()
            .fold(Vec::new(), |mut els, bucket| {
                let mut b = (*bucket).clone();
                els.append(&mut b);
                els
            });

        write!(f, "{:?}", elements)
    }
}

impl<T: Debug + PartialEq + Copy> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = Self {
            elements: vec![Vec::new(); DEFAULT_CAPACITY],
            size: 0,
            capacity: DEFAULT_CAPACITY
        };

        for element in input.iter() {
            set.add(*element);
        }

        set
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn hash(&self, element: &T) -> usize {
        let str = format!("{element:?}");
        let length = str.len();

        let hash_sum = str.chars()
            .enumerate()
            .fold(0, |sum, (i, ch)| {
                sum + (ch as usize) * 31_usize.pow((length - i) as u32)
            });

        hash_sum % self.capacity
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements[self.hash(element)]
            .iter()
            .any(|item| *item == *element)
    }

    pub fn add(&mut self, element: T) {
        let hash = self.hash(&element);
        let not_in_set = !self.elements[hash]
            .iter()
            .any(|item| *item == element);

        if not_in_set {
            self.elements[hash].push(element);
            self.size += 1;
        }
    }

    pub fn resize(&mut self) {
        let mut elements = Vec::with_capacity(self.size);

        for i in 0..self.capacity {
            elements.append(
                self.elements.get_mut(i).unwrap()
            );
        }

        self.capacity = largest_prime(self.capacity * 3);
        self.elements.resize(self.capacity, Vec::new());
        self.size = 0;

        for element in elements {
            self.add(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.len() == 0 || self.elements
            .iter()
            .all(|bucket| other.bucket_subset(bucket))
    }

    pub fn bucket_subset(&self, bucket: &Vec<T>) -> bool {
        bucket
            .iter()
            .all(|element| self.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.elements
            .iter()
            .all(|bucket| other.bucket_disjoint(bucket))
    }

    pub fn bucket_disjoint(&self, bucket: &Vec<T>) -> bool {
        bucket
            .iter()
            .all(|element| !self.contains(element))
    }

    pub fn filter<F: FnMut(&Self, &Vec<T>) -> Vec<T>>(&self, other: &Self, mut filterer: F) -> Vec<T> {
        let filtered: Vec<T> = self.elements
            .iter()
            .fold(Vec::new(), |mut common, bucket| {
                common.append(&mut filterer(other, bucket));
                common
            });

        filtered
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let intersect = self.filter(other, Self::bucket_intersection);
        Self::new(&intersect)
    }

    pub fn bucket_intersection(&self, bucket: &Vec<T>) -> Vec<T> {
        bucket
            .iter()
            .filter_map(|element| if self.contains(element) {
                Some(*element)
            } else {
                None
            }).collect()
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut diff = self.filter(other, Self::bucket_difference);
        Self::new(&diff)
    }

    pub fn bucket_difference(&self, bucket: &Vec<T>) -> Vec<T> {
        bucket
            .iter()
            .filter_map(|element| if !self.contains(element) {
                Some(*element)
            } else {
                None
            }).collect()
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut  u = self.filter(other, Self::bucket_all);
        u.append(&mut other.filter(self, Self::bucket_all));
        Self::new(&u)
    }

    pub fn bucket_all(&self, bucket: &Vec<T>) -> Vec<T> {
        bucket
            .iter()
            .map(|&element| element)
            .collect()
    }
}

fn largest_prime(limit: usize) -> usize {
    match limit {
        0|1 => 0,
        2|3 => limit,
        _ => {
            let sqrt = (limit as f64).sqrt() as usize;
            let mut primes = vec![true; limit - 1];

            for i in 2..=sqrt {
                if primes[i - 2] {
                    for j in 2..=limit / i {
                        primes[(i * j) - 2] = false;
                    }
                }
            }

            (2..=limit)
                .rev()
                .find(|&number| primes[number - 2])
                .unwrap()
        }
    }
}