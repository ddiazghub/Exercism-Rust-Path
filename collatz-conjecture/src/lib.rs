pub fn collatz(n: u64) -> Option<u64> {
    let mut n: u128 = n as u128;
    let mut i: u64 = 0;

    loop {
        if n == 1 {
            break Some(i);
        }

        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };

        i += 1;

        if n == 0 || n > u64::MAX as u128 {
            break None;
        }
    }
}
