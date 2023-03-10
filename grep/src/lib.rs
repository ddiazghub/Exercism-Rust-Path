use std::fs::{self, File};
use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt

#[derive(Debug, Copy, Clone)]
pub struct Flags {
    line_numbers: bool,
    filenames_only: bool,
    case_insensitive: bool,
    invert: bool,
    match_line: bool
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut flags_struct = Self {
            line_numbers: false,
            filenames_only: false,
            case_insensitive: false,
            invert: false,
            match_line: false
        };

        for flag in flags {
            let mut chars = flag.chars();

            match chars.next() {
                Some('-') => match chars.next() {
                    Some('n') => flags_struct.line_numbers = true,
                    Some('l') => flags_struct.filenames_only = true,
                    Some('i') => flags_struct.case_insensitive = true,
                    Some('v') => flags_struct.invert = true,
                    Some('x') => flags_struct.match_line = true,
                    _ => panic!("Invalid flag")
                },
                _ => panic!("Invalid flag")
            }
        };

        flags_struct
    }

    pub fn do_match_line(&self, pattern: &str, line: &str) -> bool {
        if self.case_insensitive {
            (pattern == line.to_lowercase()) ^ self.invert
        } else {
            (pattern == line) ^ self.invert
        }
    }

    pub fn find_pattern(&self, pattern: &str, text: &str) -> bool {
        if self.case_insensitive {
            text.to_lowercase().contains(pattern) ^ self.invert
        } else {
            text.contains(pattern) ^ self.invert
        }
    }
}
// A B X
// 0 0 0
// 0 1 1
// 1 0 1
// 1 1 0

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();

    for file in files {
        let text = fs::read_to_string(file)?;

        let mut pattern = if flags.case_insensitive {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };

        let mut lines = text.lines().enumerate();

        if flags.filenames_only {
            if flags.find_pattern(&pattern, &text) {
                result.push(file.to_string())
            }
        } else {
            let matches: Vec<_> = if flags.match_line {
                lines
                    .filter(|(i, line)| flags.do_match_line(&pattern, line)).collect()
            } else {
                lines
                    .filter(|(i, line)| flags.find_pattern(&pattern, line)).collect()
            };

            for (i, m) in matches {
                let mut line = String::new();

                if files.len() > 1 {
                    line.push_str(file);
                    line.push(':');
                }

                if flags.line_numbers {
                    line.push_str(&(i + 1).to_string());
                    line.push(':');
                }

                line.push_str(m);
                result.push(line)
            }
        }
    }

    Ok(result)
}
