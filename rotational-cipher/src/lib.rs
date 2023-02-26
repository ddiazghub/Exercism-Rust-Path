const A: u32 = 65;
const A_: u32 = 97;

pub fn rotate(input: &str, mut key: i8) -> String {
    key = key % 26;

    let ciphertext = input.chars().map(|ch| {
        let shifted = match ch {
            'a'..='z' => A_ + (((ch as u32 - A_) as i8 + key) % 26) as u32,
            'A'..='Z' => A + (((ch as u32 - A) as i8 + key) % 26) as u32,
            _ => ch as u32
        };

        char::from_u32(shifted).unwrap()
    }).collect::<String>();

    ciphertext
}
