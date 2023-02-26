pub fn get_match(ch: char) -> char {
    match ch {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        _ => '\0'
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let mut matched = true;

    for ch in string.chars() {
        match ch {
            '['|'{'|'(' => stack.push(ch),
            ']'|'}'|')' => {
                if Some(get_match(ch)) != stack.pop() {
                    matched = false;
                    break;
                }
            },
            _ => ()
        }
    }

    matched && stack.len() == 0
}
