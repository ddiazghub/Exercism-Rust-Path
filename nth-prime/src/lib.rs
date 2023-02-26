pub fn nth(n: u32) -> u32 {
    let upper_bound = estimate_nth(n);
    let mut primes = vec![true; upper_bound as usize - 1];
    let mut k = 0;


    for i in 2..=upper_bound {
        if primes[i - 2] {
            if k == n {
                return i as u32;
            }

            for j in 2..=upper_bound as usize / i {
                primes[(i * j) - 2] = false;
            }

            k += 1;
        }
    }

    0
}

pub fn estimate_nth(n: u32) -> usize {
    4 + 12 * n as usize
}