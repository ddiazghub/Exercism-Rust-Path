pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    match upper_bound {
        0|1 => vec![],
        2 => vec![2],
        3 => vec![3],
        _ => {
            let sqrt = (upper_bound as f64).sqrt() as usize;
            let mut primes = vec![true; upper_bound as usize - 1];

            for i in 2..=sqrt {
                if primes[i - 2] {
                    for j in 2..=upper_bound as usize / i {
                        primes[(i * j) - 2] = false;
                    }
                }
            }

            (2..=upper_bound)
                .filter(|&number| primes[number as usize - 2])
                .collect()
        }
    }
}