use std::collections::HashSet;

const ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats
];

pub struct Allergies {
    allergies: HashSet<Allergen>,
    score: u32
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies = Self {
            score,
            allergies: HashSet::new()
        };

        for i in 0..8 {
            let bitmask = 1 << i;

            if bitmask & score == bitmask {
                allergies.allergies.insert(ALLERGENS[i]);
            }
        }

        allergies
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies
            .iter()
            .map(|allergen| allergen.clone())
            .collect()
    }
}
