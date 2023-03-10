pub fn is_valid_isbn(isbn: &str) -> bool {
    let (count, sum) = isbn.chars()
        .filter_map(|ch: char| match ch {
            '0'..='9' => Some(ch.to_digit(10).unwrap()),
            'X' => Some(10),
            _ => None
        })
        .fold((0, 0), |(count, sum), digit| {
            if digit == 10 && count < 9 {
                (0, 0)
            } else {
                (count + 1, sum + digit * (10 - count))
            }
        });

    count == 10 && sum % 11 == 0
}