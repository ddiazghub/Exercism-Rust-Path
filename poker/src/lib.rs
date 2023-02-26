use std::cmp::Ordering;
use std::collections::HashMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum PokerCardSuit {
    Club,
    Diamond,
    Heart,
    Spade
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum PokerHand {
    StraightFlush(u8),
    FourOfAKind(u8, u8),
    FullHouse(u8, u8),
    Flush(Vec<u8>),
    Straight(u8),
    ThreeOfAKind(u8, Vec<u8>),
    TwoPair(u8, u8, u8),
    Pair(u8, Vec<u8>),
    HighCard(Vec<u8>)
}

impl PokerHand {
    pub fn new(cards: &str) -> Result<Self, ()> {
        let mut c: Vec<PokerCard> = cards
            .split(' ')
            .map(|card| PokerCard::new(card).unwrap())
            .collect();

        if c.len() != 5 {
            return Err(());
        }

        c.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        let temp_hand = Self::check_flush_straight(&mut c[..]);

        let hand = match temp_hand {
            Some(PokerHand::StraightFlush(highest)) => PokerHand::StraightFlush(highest),
            _ => {
                let frequencies = Self::value_frequency(&c[..]);
                let hand2 = Self::check_4_kind_full_house(&frequencies[..]);

                match hand2 {
                    Some(h) => h,
                    None => match temp_hand {
                        Some(h) => h,
                        None => match Self::check_else(&frequencies[..]) {
                            Some(h) => h,
                            None => return Err(())
                        }
                    }
                }
            }
        };

        Ok(hand)
    }


    pub fn check_flush_straight(cards: &mut [PokerCard]) -> Option<PokerHand> {
        let mut high = cards[1];
        let mut flush = cards[2..].iter().all(|card| card.suit == high.suit);
        let mut straight = (2..5).all(|i| cards[i].value == high.value - (i - 1) as u8);

        straight = straight && if cards[0].value == 14 && high.value == 5 {
            cards[0].value = 1;
            cards.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
            true
        } else {
            cards[0].value == high.value + 1
        };

        let t = match (flush, straight) {
            (true, true) => PokerHand::StraightFlush(cards[0].value),
            (true, false) => PokerHand::Flush(cards.iter().map(|card| card.value).collect()),
            (false, true) => PokerHand::Straight(cards[0].value),
            _ => return None
        };

        Some(t)
    }

    pub fn value_frequency(cards: &[PokerCard]) -> Vec<(u8, u8)> {
        let mut freq: HashMap<u8, u8> = HashMap::new();

        for card in cards.iter() {
            match freq.get_mut(&card.value) {
                Some(f) => *f += 1,
                None => drop(freq.insert(card.value, 1))
            };
        }

        let mut freq: Vec<(u8, u8)> = freq
            .into_iter()
            .collect();

        freq.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => b.0.cmp(&a.0),
            ord => ord
        });

        freq
    }

    fn check_4_kind_full_house(frequencies: &[(u8, u8)]) -> Option<PokerHand> {
        let hand = match frequencies.len() {
            2 => match frequencies[0].1 {
                4 => Self::FourOfAKind(frequencies[0].0, frequencies[1].0),
                _ => Self::FullHouse(frequencies[0].0, frequencies[1].0)
            },
            _ => return None
        };

        Some(hand)
    }

    fn check_else(frequencies: &[(u8, u8)]) -> Option<PokerHand> {
        let hand = match frequencies.len() {
            3 => match frequencies[0].1 {
                3 => Self::ThreeOfAKind(frequencies[0].0, (&frequencies[1..]).iter().map(|(k, _)| *k).collect()),
                _ => Self::TwoPair(frequencies[0].0, frequencies[1].0, frequencies[2].0)
            },
            4 => Self::Pair(frequencies[0].0, (&frequencies[1..]).iter().map(|(k, _)| *k).collect()),
            5 => Self::HighCard(frequencies.iter().map(|(k, _)| *k).collect()),
            _ => return None
        };

        Some(hand)
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        let ord = match (self, other) {
            (Self::StraightFlush(a), Self::StraightFlush(b)) | (Self::Straight(a), Self::Straight(b))  => a.cmp(&b),
            (Self::StraightFlush(_), _) => Ordering::Greater,
            (_, Self::StraightFlush(_)) => Ordering::Less,

            (Self::FourOfAKind(a1, a2), Self::FourOfAKind(b1, b2)) | (Self::FullHouse(a1, a2), Self::FullHouse(b1, b2)) => match a1.cmp(&b1) {
                Ordering::Equal => a2.cmp(b2),
                ord => ord
            },
            (Self::FourOfAKind(_, _), _) => Ordering::Greater,
            (_, Self::FourOfAKind(_, _)) => Ordering::Less,

            (Self::FullHouse(_, _), _) => Ordering::Greater,
            (_, Self::FullHouse(_, _)) => Ordering::Less,

            (Self::Flush(a), Self::Flush(b)) | (Self::HighCard(a), Self::HighCard(b)) => vec_cmp(a, b),
            (Self::Flush(_), _) => Ordering::Greater,
            (_, Self::Flush(_)) => Ordering::Less,

            (Self::Straight(_), _) => Ordering::Greater,
            (_, Self::Straight(_)) => Ordering::Less,

            (Self::ThreeOfAKind(a, rest_a), Self::ThreeOfAKind(b, rest_b)) | (Self::Pair(a, rest_a), Self::Pair(b, rest_b)) => match a.cmp(b) {
                Ordering::Equal => vec_cmp(rest_a, rest_b),
                ord => ord
            },
            (Self::ThreeOfAKind(_, _), _) => Ordering::Greater,
            (_, Self::ThreeOfAKind(_, _)) => Ordering::Less,

            (Self::TwoPair(a1, a2, a3), Self::TwoPair(b1, b2, b3)) => match a1.cmp(b1) {
                Ordering::Equal => match a2.cmp(b2) {
                    Ordering::Equal => a3.cmp(b3),
                    ord => ord,
                },
                ord => ord
            },
            (Self::TwoPair(_, _, _), _) => Ordering::Greater,
            (_, Self::TwoPair(_, _, _)) => Ordering::Less,

            (Self::Pair(_, _), _) => Ordering::Greater,
            (_, Self::Pair(_, _)) => Ordering::Less,
        };

        Some(ord)
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
struct PokerCard {
    pub value: u8,
    pub suit: PokerCardSuit
}

impl PokerCard {
    pub fn new(card: &str) -> Result<Self, ()> {
        if !(2..4).contains(&card.len()) {
            return Err(());
        }

        let value = Self::parse_value(&card[..card.len() - 1])?;
        let suit =  &card[card.len() - 1..].to_ascii_lowercase();

        let card = Self {
            value,
            suit: match suit.as_str() {
                "c" => PokerCardSuit::Club,
                "d" => PokerCardSuit::Diamond,
                "h" => PokerCardSuit::Heart,
                "s" => PokerCardSuit::Spade,
                _ => return Err(())
            }
        };

        Ok(card)
    }

    pub fn parse_value(string: &str) -> Result<u8, ()> {
        match string.parse() {
            Ok(n) if (2..=10).contains(&n) => Ok(n),
            _ => match string.to_ascii_lowercase().as_str() {
                "j" => Ok(11),
                "q" => Ok(12),
                "k" => Ok(13),
                "a" => Ok(14),
                _ => Err(())
            }
        }
    }

    pub fn equal_value(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for PokerCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.value.cmp(&other.value), self.suit == other.suit) {
            (Ordering::Equal, false) => None,
            (ord, _) => Some(ord)
        }
    }
}

fn vec_cmp<T: Ord>(a: &Vec<T>, b: &Vec<T>) -> Ordering {
    for (a_i, b_i) in a.iter().zip(b.iter()) {
        match a.cmp(b) {
            Ordering::Equal => (),
            other_ord => return other_ord
        }
    }

    Ordering::Equal
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands: Vec<(&str, PokerHand)> = hands
        .iter()
        .filter_map(|h| match PokerHand::new(*h) {
            Ok(h2) => Some((*h, h2)),
            _ => None
        })
        .collect();

    if hands.len() == 0 {
        return Vec::new();
    }

    let winners = (&hands[1..])
        .iter()
        .fold(vec![(hands[0].0, &hands[0].1)], |mut winners, (hand_str, hand)| {
            match winners[0].1.partial_cmp(hand).unwrap_or(Ordering::Equal) {
                Ordering::Less => return vec![(*hand_str, hand)],
                Ordering::Equal => winners.push((*hand_str, hand)),
                _ => ()
            };

            winners
        });

        winners
            .into_iter()
            .map(|(winner_str, _)| winner_str)
            .collect()
}
