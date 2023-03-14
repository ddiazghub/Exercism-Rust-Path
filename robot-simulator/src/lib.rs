use std::ops::Add;

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn advance(&self) -> Point {
        match self {
            Self::North => Point(0, 1),
            Self::South => Point(0, -1),
            Self::East => Point(1, 0),
            Self::West => Point(-1, 0)
        }
    }
    
    pub fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South
        }
    }
}

pub struct Robot {
    position: Point,
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: Point(x, y),
            direction: d
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            position: self.position,
            direction: self.direction.turn_right()
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            position: self.position,
            direction: self.direction.turn_left()
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            position: self.position + self.direction.advance(),
            direction: self.direction
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                _ => robot.advance()
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.0, self.position.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
