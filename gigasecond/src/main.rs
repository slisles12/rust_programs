use chrono::{DateTime, TimeZone, Utc};
use std::io;
fn main() {

    let mut year = String::new();
    
    println!("Please enter a year: ");

    io::stdin().read_line(&mut year).expect("failed to read from stdin");

    let year : i32= year.trim().parse().expect("invalid input");

    let mut month = String::new();
    
    println!("Please enter a month: ");

    io::stdin().read_line(&mut month).expect("failed to read from stdin");

    let month : u32= month.trim().parse().expect("invalid input");

    let mut day = String::new();
    
    println!("Please enter a day: ");

    io::stdin().read_line(&mut day).expect("failed to read from stdin");

    let day : u32= day.trim().parse().expect("invalid input");
    
    let mut hour = String::new();
    
    println!("Please enter a hour: ");

    io::stdin().read_line(&mut hour).expect("failed to read from stdin");

    let hour : u32= hour.trim().parse().expect("invalid input");
    
    let mut minute = String::new();
    
    println!("Please enter a minute: ");

    io::stdin().read_line(&mut minute).expect("failed to read from stdin");

    let minute : u32= minute.trim().parse().expect("invalid input");

    let mut second = String::new();
    
    println!("Please enter a second: ");

    io::stdin().read_line(&mut second).expect("failed to read from stdin");

    let second : u32= second.trim().parse().expect("invalid input");

    let start_date = Utc.ymd(year, month, day).and_hms(hour, minute, second);

    println!("One gigasecond after {}-{}-{}  {}:{}:{} UTC is {} ", year, month, day, hour, minute, second, one_gig_later(start_date));

}

fn one_gig_later(start: DateTime<Utc>) -> DateTime<Utc> {
    Utc.timestamp(start.timestamp() + 1000000000, 0)
}

