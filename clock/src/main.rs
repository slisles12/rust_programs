use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;


fn main() {

    let mut hour = String::new();

    println!("Please enter a hour: ");

    io::stdin().read_line(&mut hour).expect("failed to read from stdin");

    let hour : i32 = hour.trim().parse().expect("invalid input");

    let mut min = String::new();

    println!("Please enter a minute: ");

    io::stdin().read_line(&mut min).expect("failed to read from stdin");

    let min : i32 = min.trim().parse().expect("invalid input");

    println!("{}", Clock::new(hour, min).to_string());
}

pub struct Clock {
    hours: i32,
    minutes: i32,
}


impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Clock {
            hours: (total_minutes / 60) % 24,
            minutes: (minutes % 60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}