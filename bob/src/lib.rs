pub fn reply(message: &str) -> &str {
    let mut chars = message.chars()
        .rev()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '?')
        .peekable();

    let next = chars.peek().map(char::clone);
    let mut uppercase = false;
    let yelling = |ch: char| ch.is_ascii_digit() || if ch.is_uppercase() {
        uppercase = true;
        true
    } else {
        false
    };

    match next {
        Some('?') => {
            if chars.skip(1).all(yelling) && uppercase {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        },
        None => "Fine. Be that way!",
        _ if chars.all(yelling) && uppercase => "Whoa, chill out!",
        _ => "Whatever."
    }
}