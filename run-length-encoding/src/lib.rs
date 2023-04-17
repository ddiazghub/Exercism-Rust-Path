use std::ops::RangeInclusive;

const NUMS: RangeInclusive<char> = '0'..='9';

pub fn encode(source: &str) -> String {
    let mut chars = source.chars().peekable();
    let mut encoded = String::new();

    while let Some(ch) = chars.next() {
        let mut count = 1;

        while let Some(&c) = chars.peek() {
            if ch != c {
                break;
            }

            chars.next();
            count += 1;
        }

        if count > 1 {
            encoded.push_str(&count.to_string());
        }

        encoded.push(ch);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut chars = source.chars().peekable();
    let mut decoded = String::new();

    while let Some(ch) = chars.next() {
        if NUMS.contains(&ch) {
            let mut count = String::new();
            count.push(ch);

            while let Some(&c) = chars.peek() {
                if !NUMS.contains(&c) {
                    break;
                }
                
                chars.next();
                count.push(c);
            }

            let count: usize = count.parse().unwrap();
            decoded.push_str(&chars.next().unwrap().to_string().repeat(count));
        } else {
            decoded.push(ch);
        }
    }

    decoded
}
