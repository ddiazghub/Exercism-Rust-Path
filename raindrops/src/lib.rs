static SOUNDS: [(u32, &str); 3] = [
    (3, "Pling"),
    (5, "Plang"),
    (7, "Plong")
];

pub fn raindrops(n: u32) -> String {
    let sound: String = (&SOUNDS)
        .into_iter()
        .filter_map(|&(factor, sound)| (n % factor == 0).then_some(sound))
        .collect();

    match sound.len() {
        0 => n.to_string(),
        _ => sound
    }
}
