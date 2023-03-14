use std::{collections::HashMap, ptr};

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

static ZERO: [&str; 4] = [
    " _ ",
    "| |",
    "|_|",
    "   "
];

static ONE: [&str; 4] = [
    "   ",
    "  |",
    "  |",
    "   "
];

static TWO: [&str; 4] = [
    " _ ",
    " _|",
    "|_ ",
    "   "
];

static THREE: [&str; 4] = [
    " _ ",
    " _|",
    " _|",
    "   "
];

static FOUR: [&str; 4] = [
    "   ",
    "|_|",
    "  |",
    "   "
];

static FIVE: [&str; 4] = [
    " _ ",
    "|_ ",
    " _|",
    "   "
];

static SIX: [&str; 4] = [
    " _ ",
    "|_ ",
    "|_|",
    "   "
];

static SEVEN: [&str; 4] = [
    " _ ",
    "  |",
    "  |",
    "   "
];

static EIGHT: [&str; 4] = [
    " _ ",
    "|_|",
    "|_|",
    "   "
];

static NINE: [&str; 4] = [
    " _ ",
    "|_|",
    " _|",
    "   "
];

pub enum State<'a> {
    Next(HashMap<&'a str, State<'a>>),
    Final(&'a str)
}

impl<'a> State<'a> {
    fn add_chain(&mut self, transitions: &'a [&'a str], current: usize, result: &'a str) -> bool {
        match self {
            Self::Next(t) => {
                if current == transitions.len() - 1 {
                    t.insert(transitions[current], State::Final(result));
                    true
                } else {
                    match t.get_mut(transitions[current]) {
                        Some(next) => next.add_chain(transitions, current + 1, result),
                        None => {
                            let mut next = State::Next(HashMap::new());
                            let result = next.add_chain(transitions, current + 1, result);
                            t.insert(transitions[current], next);
                            result
                        }
                    }
                }
            },
            Self::Final(_) => false
        }
    }
}

pub struct ParseStateMachine<'a> {
    //transitions: HashMap<&'a str, HashMap<(u32, &'a str), State<'a>>>,
    initial: State<'a>,
    current: *const State<'a>,
    default_string: &'a str,
    default: State<'a>,
}

impl<'a> ParseStateMachine<'a> {
    pub fn new() -> Self {
        let initial = State::Next(HashMap::from([("", State::Final(""))]));

        let mut this = Self {
            current: ptr::null(),
            initial,
            default_string: "",
            default: State::Final("")
        };

        this.current = &this.initial as *const State<'a>;
        this
    }

    // Can't be externally mutated after the first transition is made
    pub fn mutable(&self) -> bool {
        self.current == &self.initial as *const State<'a>
    }

    pub fn set_default(&mut self, default: &'a str) {
        if self.mutable() {
            self.default_string = default;
            self.default = State::Final(default);
        }
    }

    pub fn add_chain(&mut self, transitions: &'a [&'a str], result: &'a str) -> bool {
        self.mutable() && self.initial.add_chain(transitions, 0, result)
    }

    pub fn get_result(&self) -> Option<&str> {
        unsafe {
            match self.current.as_ref() {
                Some(State::Final(result)) => Some(result),
                _ => None 
            }
        }
    }

    pub fn transition<'b>(&mut self, next: &'b str) -> Option<&'a str> {
        unsafe {
            match self.current.as_ref() {
                Some(State::Final(result)) => Some(result),
                Some(State::Next(transitions)) => {
                    match transitions.get(next) {
                        Some(new_state) => {
                            self.current = new_state as *const State<'a>;

                            match new_state {
                                &State::Final(result) => Some(result),
                                _ => None
                            }
                        },
                        None => {
                            self.current = &self.default;
                            Some(self.default_string)
                        }
                    }
                },
                None => Some(self.default_string)
            }
        }
    }

    pub fn ocr_parser() -> Self {
        let mut ocr_parser = Self::new();
        ocr_parser.set_default("?");

        ocr_parser.add_chain(&ZERO, "0");
        ocr_parser.add_chain(&ONE, "1");
        ocr_parser.add_chain(&TWO, "2");
        ocr_parser.add_chain(&THREE, "3");
        ocr_parser.add_chain(&FOUR, "4");
        ocr_parser.add_chain(&FIVE, "5");
        ocr_parser.add_chain(&SIX, "6");
        ocr_parser.add_chain(&SEVEN, "7");
        ocr_parser.add_chain(&EIGHT, "8");
        ocr_parser.add_chain(&NINE, "9");

        ocr_parser
    }
}

impl<'a> Default for ParseStateMachine<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StrChunks<'a> {
    slice: &'a str,
    chunk_size: usize
}

impl<'a> StrChunks<'a> {
    pub fn new(slice: &'a str, chunk_size: usize) -> Self {
        Self {
            slice,
            chunk_size
        }
    }
}

impl<'a> Iterator for StrChunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        (self.slice.len() > 0)
            .then(|| {
                let (chunk, new_slice) = match self.slice.char_indices().skip(self.chunk_size).next() {
                    None => (&self.slice[..], ""),
                    Some((i, _)) => (&self.slice[..i], &self.slice[i..])
                };

                self.slice = new_slice;
                chunk
            })
    }
}

pub trait StrChunksIterator<'a> {
    fn chunks(&'a self, chunk_size: usize) -> StrChunks<'a>;
}

impl<'a> StrChunksIterator<'a> for str {
    fn chunks(&'a self, chunk_size: usize) -> StrChunks<'a> {
        StrChunks::new(self, chunk_size)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut parsers: Vec<Vec<ParseStateMachine>> = Vec::new();
    let mut line_count = 0;

    for line in input.split('\n') {
        let row = line_count / 4;
        let mut char_count = 0;

        if line_count % 4 == 0 {
            parsers.push(Vec::new())
        }

        for chunk in line.chunks(3) {
            let col = char_count / 3;

            if char_count % 3 == 0 && line_count % 4 == 0  {
                parsers[row].push(ParseStateMachine::ocr_parser());
            }

            parsers[row][col].transition(chunk);
            char_count += chunk.chars().count();
        }

        if char_count % 3 != 0 {
            return Err(Error::InvalidColumnCount(char_count));
        }

        line_count += 1;
    }

    if line_count % 4 != 0 {
        return Err(Error::InvalidRowCount(line_count));
    }
    
    let mut num = parsers
        .iter()
        .map(|row| row
            .into_iter()
            .map(|parser| parser.get_result().unwrap())
            .collect::<String>()
        )
        .fold(String::new(), |num, row_result| num + &row_result + ",");
    
    num.pop();
    Ok(num)
}
