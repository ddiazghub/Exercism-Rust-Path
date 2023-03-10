use std::str::Chars;

pub struct Acronym<'a> {
    chars: Chars<'a>,
    first: bool
}

impl <'a> Acronym<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars(),
            first: true
        }
    }
}

impl Iterator for Acronym<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lowercase = false;
        let mut non_alnum = false;

        while let Some(c) = self.chars.next() {
            if c.is_alphanumeric() {
                if self.first || non_alnum {
                    self.first = false;

                    return Some(c.to_ascii_uppercase());
                }

                let is_lower = c.is_lowercase();

                if is_lower {
                    lowercase = true;
                } else {
                    if lowercase {
                        return Some(c);
                    }
                }
            } else if c != '\'' {
                non_alnum = true;
            }
        }

        None
    }
}

pub trait AcronymIterator {
    fn acronym(&self) -> Acronym;
}

impl AcronymIterator for str {
    fn acronym(&self) -> Acronym {
        Acronym::new(self)
    }
}

pub fn abbreviate(phrase: &str) -> String {
    phrase.acronym().collect()
}
