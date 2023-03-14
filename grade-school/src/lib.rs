use std::collections::{BTreeMap, BTreeSet};

pub struct Grade<'a>(BTreeSet<&'a str>);

impl<'a> Grade<'a> {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn add(&mut self, student: &'a str) {
        self.0.insert(student);
    }
}

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School<'a>(BTreeMap<u32, Grade<'a>>);

impl<'a> School<'a> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        self.0.entry(grade).or_insert(Grade::new()).add(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.0.get(&grade) {
            None => Vec::new(),
            Some(Grade(students)) => students.iter()
                .map(|student| student.to_string())
                .collect()
        }
    }
}
