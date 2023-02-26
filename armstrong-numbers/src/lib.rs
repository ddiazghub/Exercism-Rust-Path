pub fn is_armstrong_number(num: u32) -> bool {
    let number = num.to_string();
    let n = number.len() as u32;

    let sum: u32 = number.chars().map(|digit| {
        let d = digit.to_digit(10).unwrap();
        d.pow(n)
    }).sum();

    sum == num
}
