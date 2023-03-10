use std::fmt::{Display, Formatter, Result, write};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let st = self.0.to_string();

        let roman: String = st
            .chars()
            .enumerate()
            .fold(String::new(), |s, (i, digit)| {
                s + &match st.len() - i  {
                    1 => match_symbols(digit, ("I", "V", "X")),
                    2 => match_symbols(digit, ("X", "L", "C")),
                    3 => match_symbols(digit, ("C", "D", "M")),
                    4 => "M".repeat(digit.to_digit(10).unwrap() as usize),
                    _ => String::from("")
                }
            });

        write!(_f, "{}", roman)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(num)
    }
}

pub fn match_symbols(digit: char, symbols: (&str, &str, &str)) -> String {
    let val = digit.to_digit(10).unwrap() as usize;

    match digit {
        ch if ('1'..'4').contains(&ch) => symbols.0.repeat(val),
        '4' => String::from(symbols.0) + symbols.1,
        ch if ('5'..'9').contains(&ch) => String::from(symbols.1) + &symbols.0.repeat(val - 5),
        '9' => String::from(symbols.0) + symbols.2,
        _ => String::from("")
    }
}