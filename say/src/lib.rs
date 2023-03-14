use std::iter::{Peekable, Take};

static LARGE_NUMBERS: [&str; 7] = ["",
    "thousand ",
    "million ",
    "billion ",
    "trillion ",
    "quadrillion ",
    "quintillion "
];

static TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen"
];

static TENTHS: [&str; 10] = [
    "",
    "",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety"
];

static UNITS: [&str; 10] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

struct IterChunk<I>(I);

impl <I: Iterator> Iterator for IterChunk<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

struct IterChunks<I: Iterator> {
    iter: Peekable<I>,
    chunk_size: usize
}

impl <I: Iterator + Clone> Iterator for IterChunks<I> where I::Item: Clone {
    type Item = IterChunk<Take<Peekable<I>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.iter.peek();

        if value.is_some() {
            let mut iter = self.iter
                .clone()
                .take(self.chunk_size);

            for i in 0..self.chunk_size {
                self.iter.next();
            }

            Some(IterChunk(iter))
        } else {
            None
        }
    }
}

trait IterChunksIterator<> {
    fn chunks(self, chunk_size: usize) -> IterChunks<Self> where Self: Sized + Iterator;
}

impl <I: Iterator + Clone> IterChunksIterator for I {
    fn chunks(self, chunk_size: usize) -> IterChunks<Self> {
        IterChunks {
            iter: self.peekable(),
            chunk_size
        }
    }
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        String::from("zero")
    } else {
        let string = n.to_string();
        let num_digits = string.chars().count();
        let padding = (3 - num_digits % 3) % 3;
        let padded = "0".repeat(padding) + &string;
        let length = (padding + num_digits) / 3;
        println!("l: {length}, p: {padded}");

        let s: String = padded.chars()
            .chunks(3)
            .enumerate()
            .map(|(i, chunk)| {
                let mut part1 = encode_chunk(chunk);

                if part1.len() > 0 {
                    part1.push_str(LARGE_NUMBERS[length - (i + 1)]);
                }

                part1
            })
            .collect();

        s.trim_end().to_string()
    }
}

fn encode_chunk<I: Iterator<Item=char>>(mut chunk: IterChunk<I>) -> String {
    let mut part1 = UNITS[chunk.next().unwrap().to_digit(10).unwrap() as usize].to_string();
    let second = chunk.next().unwrap().to_digit(10).unwrap() as usize;
    let third = chunk.next().unwrap().to_digit(10).unwrap() as usize;

    let part2 = match second {
        d if (2..=9).contains(&d) => {
            let mut part1 = TENTHS[d].to_string();
            let part2 = UNITS[third];

            if part2.len() > 0 {
                part1.push('-');
            }

            part1 + part2
        },
        1 => TEENS[third].to_string(),
        _ => UNITS[third].to_string()
    };

    match (part1.len() > 0, part2.len() > 0) {
        (true, true) => part1 + " hundred " + &part2 + " ",
        (true, false) => part1 + " hundred ",
        (false, true) => part2 + " ",
        _ => part1
    }
}