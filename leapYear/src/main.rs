use std::io; 

fn main() {

    let mut year = String::new();

    println!("Please enter a year: ");

    io::stdin().read_line(&mut year).expect("failed to read from stdin");

    let year : i32 = year.trim().parse().expect("invalid input");

    if year % 4 != 0 {
        println!("{} is not a leap year!", year);
    }
    else{
        println!("{} is a leap year!", year);
    }
}
