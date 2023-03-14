use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl BucketStats {
    pub fn new(moves: u8, goal_bucket: Bucket, other_bucket: u8) -> Self {
        Self {
            moves,
            goal_bucket,
            other_bucket
        }
    }
}

fn pour((liters1, liters2): (u8, u8), (cap1, cap2): (u8, u8), first: bool) -> (u8, u8) {
    if first {
        let amount = liters1.min(cap2 - liters2);
        (liters1 - amount, liters2 + amount)
    } else {
        let amount = liters2.min(cap1 - liters1);
        (liters1 + amount, liters2 - amount)
    }
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    if goal == 0 {
        return None;
    }

    let capacities = (capacity_1, capacity_2);
    let start1 = (capacity_1, 0);
    let start2 = (0, capacity_2);

    let mut moves_queue = VecDeque::new();
    let mut moves_done = HashSet::new();

    let (first_move, forbidden) = match start_bucket {
        &Bucket::One => (start1, start2),
        &Bucket::Two => (start2, start1)
    };

    moves_queue.push_back((1, first_move));
    moves_done.insert((0, 0));
    moves_done.insert(forbidden);

    while let Some((moves, liters)) = moves_queue.pop_front() {
        match true {
            _ if moves_done.contains(&liters) => continue,
            _ if goal == liters.0 => return Some(BucketStats::new(moves, Bucket::One, liters.1)),
            _ if goal == liters.1 => return Some(BucketStats::new(moves, Bucket::Two, liters.0)),
            _ => {
                let next = moves + 1;

                moves_done.insert(liters);
                moves_queue.push_back((next, pour(liters, capacities, true)));
                moves_queue.push_back((next, pour(liters, capacities, false)));
                moves_queue.push_back((next, (0, liters.1)));
                moves_queue.push_back((next, (liters.0, 0)));
                moves_queue.push_back((next, (capacity_1, liters.1)));
                moves_queue.push_back((next, (liters.0, capacity_2)));
            }
        }
    }
    
    None
}
