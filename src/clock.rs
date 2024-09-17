use std::fmt::{Display, Formatter, Result};

fn main() {
    let clock = Clock::new(0, 3).add_minutes(-4);

    println!("{}", clock);
}

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Handling when hours are negative and less than -24
        let hours_to_add = hours % 24;

        let new_hours = (24 + hours_to_add) % 24;

        let clock = Self {
            hours: new_hours,
            minutes: 0,
        };

        clock.add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours_to_add = minutes / 60;

        let mut new_hours = self.hours + hours_to_add;

        let minutes_to_add = minutes % 60;
        let mut new_minutes = self.minutes + minutes_to_add;

        if new_hours < 0 {
            new_hours = 24 + new_hours % 24;
        }

        if new_minutes >= 60 {
            new_hours += 1;
            new_minutes = new_minutes - 60;
        }

        if new_minutes < 0 {
            if new_hours != 0 {
                new_hours -= 1;
            } else {
                // Handle substract across midnight
                new_hours = 23;
            }
            new_minutes = 60 + new_minutes;
        }

        Self {
            hours: new_hours % 24,
            minutes: new_minutes
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
