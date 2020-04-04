use std::io;
fn main() {
    let mut number = String::new();
    
    println!("Please enter a number: ");

    io::stdin().read_line(&mut number).expect("failed to read from stdin");

    let number : i32 = number.trim().parse().expect("invalid input");

    if (number > 64) || (number < 1){
        panic!("Sqaure must be between 1 and 64")
    }

    println!("The total number of grains on the board is {}", calculate_total_board());
    println!("The total number of grains on sqaure {} is {}", number, calculate_given_square(number));

}

fn calculate_given_square(x: i32) -> u64 {

    let mut sum = 1 as u64;

    for i in 1..x {
        sum = sum * 2;
    }

    return sum as u64;

}

fn calculate_total_board() -> u64 {
    let mut sum = 1 as u64;

    for i in 1..64 {
        sum = sum * 2;
    }

    return sum;
}