use std::collections::HashMap;
use std::ops::Sub;

const PRICE: u32 = 800;

static DISCOUNTED: [f64; 5] = [
    1.0,
    0.95,
    0.90,
    0.80,
    0.75
];

trait SetsOf {
    fn sets_of(&self, n: u32) -> HashMap<u32, u32>;
}

impl<T: Copy + Clone + PartialOrd + PartialEq + Sub<Output=T> + From<u8>> SetsOf for Vec<T> {
    fn sets_of(&self, n: u32) -> HashMap<u32, u32> {
        let zero = T::from(0);
        let one = T::from(1);
        let mut elements: Vec<T> = self.clone();
        let mut sets = HashMap::new();

        while elements.len() > 0 {
            let counted = elements
                .into_iter()
                .fold((0, Vec::new()), move |(count, mut elements), element| {
                    match (element > zero, count < n) {
                        (true, true) => {
                            if element - one > zero {
                                elements.push(element - one);
                            }
    
                            (count + 1, elements)
                        },
                        (true, false) => {
                            elements.push(element);
                            (count, elements)
                        }
                        _ => (count, elements),
                    }
                });

            *sets.entry(counted.0).or_insert(0) += 1;
            elements = counted.1;
        }

        sets
    }
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts: Vec<u32> = books
        .into_iter()
        .fold(vec![0; 5], |mut counts, &book| {
            counts[book as usize - 1] += 1;
            counts
        });
    
    let price1 = apply_discount_to_sets(counts.sets_of(5));
    let price2 = apply_discount_to_sets(counts.sets_of(4));

    price1.min(price2)
}

fn apply_discount_to_sets(set: HashMap<u32, u32>) -> u32 {
    set
        .into_iter()
        .map(|(set_size, count)| apply_discount(set_size) * count)
        .sum()
}

fn apply_discount(num_books: u32) -> u32 {
    if num_books > 0 {
        ((num_books * PRICE) as f64 * DISCOUNTED[num_books as usize - 1]) as u32
    } else {
        0
    }
}