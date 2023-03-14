#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Vec<u16>>,
    in_frame: bool,
    bonus_rolls: u16
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            in_frame: false,
            bonus_rolls: 0
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let mut frame = self.frames.len();

        if self.in_frame {
            let rolls = self.frames[frame - 1].len();
            
            if 10 - self.frames[frame - 1][rolls - 1] < pins {
                return Err(Error::NotEnoughPinsLeft);
            }

            self.frames[frame - 1].push(pins);
            self.in_frame = false;

            if frame == 10 && self.frames[9][0] + self.frames[9][1] == 10 {
                self.bonus_rolls = 1;
            }
        } else if self.bonus_rolls > 0 {
            let rolls = self.frames[frame - 1].len();
            let frame_rolls = &self.frames[frame - 1];

            if rolls == 2 && frame_rolls[0] == 10 && frame_rolls[1] < 10 && (10 - frame_rolls[rolls - 1] < pins) {
                return Err(Error::NotEnoughPinsLeft);
            }
            
            self.frames[9].push(pins);
            self.bonus_rolls -= 1;
        } else if frame == 10 {
            return Err(Error::GameComplete);
        } else {
            self.frames.push(vec![pins]);
            frame += 1;

            match pins {
                10 => match frame {
                    10 => self.bonus_rolls = 2,
                    _ => ()
                } 
                _ => self.in_frame = true
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.in_frame && self.frames.len() == 10 && self.bonus_rolls == 0 {
            let score = self.frames
                .iter()
                .enumerate()
                .map(|(i, frame)| match i {
                    9 => frame.iter().copied().sum(),
                    _ => match frame.len() {
                        1 => 10 + match self.frames[i + 1].len() {
                            1 => self.frames[i + 1][0] + self.frames[i + 2][0],
                            _ => self.frames[i + 1][0] + self.frames[i + 1][1]
                        },
                        _ => match frame[0] + frame[1] {
                            10 => 10 + self.frames[i + 1][0],
                            frame_score => frame_score 
                        }
                    }
                })
                .sum();
            
                Some(score)
        } else {
            None
        }

    }
}
