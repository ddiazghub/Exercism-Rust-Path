pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut substrings = Vec::new();
    let string_length = digits.len();

    if len <= string_length {
        for i in 0..=string_length - len {
            substrings.push(digits[i..i + len].to_string());
        }
    }

    substrings
}
