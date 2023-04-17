use std::collections::HashMap;

thread_local! {
    static CHILDREN: HashMap<&'static str, usize> = [
        "Alice", "Bob", "Charlie", "David",
        "Eve", "Fred", "Ginny", "Harriet",
        "Ileana", "Joseph", "Kincaid", "Larry"
    ].into_iter()
        .zip(0..)
        .collect();

    static PLANTS: HashMap<char, &'static str> = HashMap::from([
        ('V', "violets"),
        ('R', "radishes"),
        ('C', "clover"),
        ('G', "grass")
    ]);
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let (student_id, len) = CHILDREN.with(|it| (it[student], it.len()));

    diagram.split('\n')
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .skip(student_id)
                .enumerate()
                .filter(|&(j, _)| (student_id..=student_id + 1).contains(&(j % (len * 2))))
                .map(move |(_, plant)| PLANTS.with(|it| {
                    it[&plant]
                }))
        }).collect()
}