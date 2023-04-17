use std::collections::HashSet;
use std::ops::Index;

pub struct Item {
    pub weight: u32,
    pub value: u32,
}

impl Item {
    pub fn score(&self) -> f64 {
        self.value as f64 / self.weight as f64
    }
}

pub fn maximum_value(max_weight: u32, mut items: Vec<Item>) -> u32 {
    items.sort_unstable_by(|item1, item2| item2.score().partial_cmp(&item1.score()).unwrap());

    match items.iter().position(|item| item.weight <= max_weight) {
        Some(index) => {
            let index = items.iter().position(|item| {
                item.weight == max_weight && item.score() * 1.5 > items[index].score()
            }).unwrap_or(index);

            next_item(index, &items, &mut HashSet::new(), items[index].value, max_weight - items[index].weight)
        },
        None => 0
    }
}


pub fn next_item(current: usize, items: &Vec<Item>, taken: &mut HashSet<usize>, total: u32, remaining: u32) -> u32 {
    taken.insert(current);
    let mut max = total;
    let mut j = 0;

    for i in 0..items.len() {
        if j < items.len() / 3 && !taken.contains(&i) && items[i].weight <= remaining {
            let total_value = next_item(i, items, taken, total + items[i].value, remaining - items[i].weight);

            if total_value > max {
                max = total_value;
            }

            j += 1;
        }
    }

    taken.remove(&current);
    max
}