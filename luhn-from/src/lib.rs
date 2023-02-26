macro_rules! luhn_from {
    ( $( $types:ty ),* ) => {
        $(
            impl From<$types> for Luhn {
                fn from(input: $types) -> Self {
                    Luhn::from(input as u64)
                }
            }
        )*
    }
}

pub struct Luhn {
    number: Option<u64>,
    valid: bool
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl From<u64> for Luhn {
    fn from(input: u64) -> Self {
        if input == 0 {
            return Self {
                number: Some(0),
                valid: false
            }
        }

        let mut sw = true;
        let mut sum = 0;
        let mut number = input;

        while number > 0 {
            let digit = number % 10;

            sum += if sw {
                digit
            } else {
                let digit2 = 2 * digit;
                digit2 % 10 + digit2 / 10
            };

            sw = !sw;
            number /= 10;
        }

        Self {
            number: Some(input),
            valid: sum % 10 == 0
        }
    }
}

luhn_from!(u32, u16, u8, usize);

impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        match input.replace(" ", "").parse::<u64>() {
            Ok(n) => Luhn::from(n),
            _ => Self {
                number: None,
                valid: false
            }
        }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::from(input.as_str())
    }
}