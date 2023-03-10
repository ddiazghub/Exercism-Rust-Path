#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        span if span > string_digits.len() => Err(Error::SpanTooLong),
        _ => {
            let digits: Result<Vec<u64>, Error> = string_digits.chars()
                .map(|ch| ch.to_digit(10).map(u64::from).ok_or(Error::InvalidDigit(ch)))
                .collect();

            digits.map(|d| {
                d.windows(span)
                    .map(|window| window.into_iter().product())
                    .max()
                    .unwrap()
            })
        }
    }
}
