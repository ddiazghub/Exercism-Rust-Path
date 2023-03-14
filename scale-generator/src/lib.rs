use std::{ascii::AsciiExt, ptr};

// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

static SHARPS: [&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
static FLATS: [&str; 12] = ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];


#[derive(Debug)]
pub enum ErrorType {
    Default
}

#[derive(Debug)]
pub struct Error(ErrorType);

pub struct LoopFrom<'a, T> {
    slice: &'a [T],
    current: usize
}

impl<'a, T> LoopFrom<'a, T> {
    pub fn new(slice: &'a [T], start_index: usize) -> Self {
        if start_index < slice.len() {
            Self {
                slice,
                current: start_index
            }
        } else {
            panic!("Index out of bounds");
        }
    }
}

impl<'a, T> Iterator for LoopFrom<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.slice.len() {
            self.current = 0;
        }

        let item = self.slice.get(self.current);
        self.current += 1;
        
        item
    }
}

pub trait Loopable<T> {
    fn loop_from(&self, start_index: usize) -> LoopFrom<'_, T>;
}

impl<T> Loopable<T> for [T] {
    fn loop_from(&self, start_index: usize) -> LoopFrom<'_, T> {
        LoopFrom::new(self, start_index)
    }
}

pub struct Scale(Vec<String>);

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale = Self::get_scale(tonic)?;

        let index = if ptr::eq(scale, &SHARPS) {
            Self::sharp_index(tonic)?
        } else {
            Self::flat_index(tonic)?
        };

        let mut scale_iter = scale.loop_from(index);
        let mut scale = Self(Vec::new());

        for interval in intervals.chars() {
            scale.0.push(scale_iter.next().unwrap().to_string());

            let step = match interval {
                'm' => continue,
                'M' => 0,
                'A' => 1,
                _ => return Err(Error(ErrorType::Default))
            };

            scale_iter.nth(step);
        }

        scale.0.push(scale_iter.next().unwrap().to_string());
        Ok(scale)
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let scale = Self::get_scale(tonic)?;

        let index = if ptr::eq(scale, &SHARPS) {
            Self::sharp_index(tonic)?
        } else {
            Self::flat_index(tonic)?
        };

        let notes: Vec<String> = scale
            .loop_from(index)
            .take(13)
            .map(|note| note.to_string())
            .collect();

        Ok(Self(notes))
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }

    pub fn get_scale(tonic: &str) -> Result<&[&str; 12], Error> {
        match tonic {
            "G"|"D"|"A"|"E"|"B"|"C"|"F#"|"e"|"b"|"a"|"f#"|"c#"|"g#"|"d#" => Ok(&SHARPS),
            "F"|"Bb"|"Eb"|"Ab"|"Db"|"Gb"|"d"|"g"|"c"|"f"|"bb"|"eb" => Ok(&FLATS),
            _ => Err(Error(ErrorType::Default))
        }
    }
    pub fn sharp_index(tonic: &str) -> Result<usize, Error> {
        let tonic = tonic.to_lowercase();

        let index = (&SHARPS)
            .into_iter()
            .position(|&note| note.to_lowercase() == tonic)
            .ok_or(Error(ErrorType::Default))?;

        Ok(index)
    }

    pub fn flat_index(tonic: &str) -> Result<usize, Error> {
        let tonic = tonic.to_lowercase();

        let index = (&FLATS)
            .into_iter()
            .position(|&note| note.to_lowercase() == tonic)
            .ok_or(Error(ErrorType::Default))?;

        Ok(index)
    }
}
