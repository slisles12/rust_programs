use std::io;

fn main() {

    println!("Please input a number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line!");

    println!("your number was {}", guess);
}
