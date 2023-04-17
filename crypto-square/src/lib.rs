pub fn encrypt(input: &str) -> String {
    if input.len() == 0 {
        return input.to_string();
    }

    let (len, mut normalized) = input
        .chars()
        .fold((0_usize, String::new()), |(len, mut string), ch| {
            let new_len = if ch.is_alphanumeric() {
                string.push(ch.to_ascii_lowercase());
                len + 1
            } else {
                len
            };

            (new_len, string)
        });
    
    let sqrt = (len as f64).sqrt();
    let c = sqrt.ceil() as usize;
    let r = sqrt.floor() as usize;

    let padding = c * r - len; 
    normalized.push_str(&" ".repeat(padding));

    let mut rectangle: Vec<String> = vec![String::new(); c];
    
    for (i, ch) in normalized.chars().enumerate() {
        rectangle[i % c].push(ch);
    }

    rectangle.join(" ")
}
