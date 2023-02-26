pub fn is_leap_year(year: u64) -> bool {
    (divisible(year, 4) && !divisible(year, 100)) || divisible(year, 400)

}

pub fn divisible(number: u64, divisor: u64) -> bool {
    number % divisor == 0
}
