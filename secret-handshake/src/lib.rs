static ACTIONS: [&str; 4] = [
    "wink",
    "double blink",
    "close your eyes",
    "jump"
];

pub fn actions(n: u8) -> Vec<&'static str> {
    let iter = (0..4)
        .into_iter()
        .filter_map(|i| ((1 << i) & n > 0).then_some(ACTIONS[i]));

    match 16 & n {
        0 => iter.collect(),
        _ => iter.rev().collect()
    }
}
