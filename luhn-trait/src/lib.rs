macro_rules! impl_luhn {
    ( $( $types:ty ),* ) => {
        $(
            impl Luhn for $types {
                fn valid_luhn(&self) -> bool {
                    (*self as u64).valid_luhn()
                }
            }
        )*
    }
}

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        *self > 0 && {
            let mut sw = true;
            let mut sum = 0;
            let mut number = *self;

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

            sum % 10 == 0
        }
    }
}

impl_luhn!(u32, u16, u8, usize);

impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        match self.replace(" ", "").parse::<u64>() {
            Ok(n) => n.valid_luhn(),
            _ => false
        }
    }
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        self.as_str().valid_luhn()
    }
}
