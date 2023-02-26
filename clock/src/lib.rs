use std::fmt::{Display, Formatter, write};

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (fixed_hours, fixed_minutes) = Self::fix_time(hours, minutes);

        Self {
            hours: fixed_hours,
            minutes: fixed_minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(
            self.hours + minutes / MINUTES_PER_HOUR,
            self.minutes + minutes % MINUTES_PER_HOUR
        )
    }

    fn fix_time(hours: i32, minutes: i32) -> (i32, i32) {
        let mut extra_hours = hours + (minutes as f64 / MINUTES_PER_HOUR as f64).floor() as i32;
        let fixed_hours = (extra_hours % HOURS_PER_DAY + HOURS_PER_DAY) % HOURS_PER_DAY;
        let fixed_minutes = ((minutes % MINUTES_PER_HOUR + MINUTES_PER_HOUR) % MINUTES_PER_HOUR);
        (fixed_hours, fixed_minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
