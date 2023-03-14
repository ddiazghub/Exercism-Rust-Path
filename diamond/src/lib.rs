pub fn get_diamond(c: char) -> Vec<String> {
    let char_index = c.to_ascii_uppercase() as u8 - 'A' as u8;
    let mut diamonds = Vec::new();

    for i in 0..char_index + 1 {
        diamonds.push(diamond_row(i, char_index));
    }

    for i in (0..char_index).rev() {
        diamonds.push(diamond_row(i, char_index));
    }

    diamonds
}

pub fn diamond_row(i: u8, char_index: u8) -> String {
    let mut row = String::new();
    let ch = (i + 'A' as u8) as char;
    let spaces = " ".repeat((char_index - i) as usize);
    row.push_str(&spaces);
    row.push(ch);

    if i > 0 {
        row.push_str(&" ".repeat((2 * (i - 1) + 1) as usize));
        row.push(ch);
    }

    row.push_str(&spaces);
    
    row
}