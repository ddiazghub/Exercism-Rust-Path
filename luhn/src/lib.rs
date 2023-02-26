/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match (code.trim().len() > 1, code.replace(" ", "").parse()) {
        (true, Ok(n)) => luhn(n),
        _ => false
    }
}

fn luhn(mut number: i64) -> bool {
    let mut sw = true;
    let mut sum = 0;

    while number > 0 {
        let digit = number % 10;

        sum += if sw {
            digit
        } else {
            let digit2 = 2 * digit;

            digit2 % 10 + digit2 / 10
        };

        sw = !sw;
        number /= 10;
    }

    sum % 10 == 0
}