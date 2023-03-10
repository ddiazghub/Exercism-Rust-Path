const N_SQUARES: u32 = 64;

pub fn square(s: u32) -> u64 {
    match s {
        1..=N_SQUARES => 1 << s - 1,
        _ => panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    ((1_u128 << N_SQUARES) - 1) as u64
}
