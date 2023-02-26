// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const MIN_CARS_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        5..=8 => 0.9,
        9|10 => 0.77,
        _ => 1.0,
    };

    speed as f64 * MIN_CARS_PER_HOUR * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let success_rate = match speed {
        5..=8 => 0.9,
        9|10 => 0.77,
        _ => 1.0,
    };

    (speed as f64 * MIN_CARS_PER_HOUR * success_rate / 60.0) as u32
}
