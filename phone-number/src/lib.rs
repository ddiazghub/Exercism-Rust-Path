pub fn number(user_number: &str) -> Option<String> {
    let num: String = user_number
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .collect();

    let sliced = match num.len() {
        10 => &num[..],
        11 => {
            if num.as_bytes()[0] != '1' as u8 {
                return None
            }

            &num[1..]
        },
        _ => return None
    };

    if sliced.as_bytes()[0] < '2' as u8 || sliced.as_bytes()[3] < '2' as u8 {
        return None;
    }

    Some(sliced.to_string())
}
