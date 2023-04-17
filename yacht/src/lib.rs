use std::collections::btree_set::BTreeSet;
use std::collections::HashMap;

pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

impl Category {
    pub fn score(&self, dice: Dice) -> u8 {
        let dice = DiceStruct { dice };

        match self {
            Self::Ones => dice.sum_number(1),
            Self::Twos => dice.sum_number(2),
            Self::Threes => dice.sum_number(3),
            Self::Fours => dice.sum_number(4),
            Self::Fives => dice.sum_number(5),
            Self::Sixes => dice.sum_number(6),
            Self::Choice => dice.sum_of(|_| true),
            Self::Yacht => dice.yacht(),
            Self::FullHouse => dice.full_house(),
            Self::FourOfAKind => dice.four_of_a_kind(),
            Self::LittleStraight => dice.straight(1),
            Self::BigStraight => dice.straight(2)
        }
    }
}

type Dice = [u8; 5];

struct DiceStruct {
    dice: Dice
}

impl DiceStruct {
    fn sum_number(&self, number: u8) -> u8 {
        self.sum_of(|&&roll| roll == number)
    }

    fn sum_of(&self, predicate: impl FnMut(&&u8) -> bool) -> u8 {
        self.dice
            .iter()
            .filter(predicate)
            .sum()
    }

    fn count(&self) -> HashMap<u8, u8> {
        self.dice
            .iter()
            .fold(HashMap::new(), |mut count, roll| {
                *count.entry(*roll).or_insert(0) += 1;
                count
            })
    }

    fn four_of_a_kind(&self) -> u8 {
        match self.count().into_iter().find(|&(_, count)| count > 3) {
            Some((roll, _)) => roll * 4,
            None => 0
        }
    }

    fn full_house(&self) -> u8 {
        let counts = self.count();
        let values: Vec<u8> = counts.values().copied().collect();

        let is_full_house = [[2, 3], [3, 2]]
            .into_iter()
            .any(|it| &it == &values[..]);

        if is_full_house {
            counts.into_iter()
                .map(|(roll, count)| roll * count)
                .sum()
        } else {
            0
        }
    }

    fn straight(&self, start: u8) -> u8 {
        let rolls: BTreeSet<u8> = self.dice.iter()
            .copied()
            .collect();

        let is_straight = rolls.len() == 5 && rolls.into_iter()
            .zip(start..start + 5)
            .all(|(roll, expected)| roll == expected);

        if is_straight { 30 } else { 0 }
    }

    fn yacht(&self) -> u8 {
        if self.dice[1..].iter().all(|&roll| roll == self.dice[0]) {
            50
        } else { 0 }
    }
}

pub fn score(dice: Dice, category: Category) -> u8 {
    category.score(dice)
}