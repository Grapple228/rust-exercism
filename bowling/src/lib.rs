#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    rolls: Vec<u16>,
    is_second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.is_second && pins + self.rolls.last().unwrap() > 10) {
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(Error::GameComplete)
        } else {
            self.rolls.push(pins);
            self.is_second = if pins != 10 {
                !self.is_second
            } else {
                self.is_second
            };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut frame = 0;
        let rolls = &self.rolls;
        for _ in 0..10 {
            if let (Some(&first), Some(&second)) = (rolls.get(frame), rolls.get(frame + 1)) {
                total += first + second;
                if first == 10 || first + second == 10 {
                    if let Some(&third) = rolls.get(frame + 2) {
                        total += third;
                    } else {
                        return None;
                    }
                }
                frame += if first == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }
        Some(total)
    }
}
