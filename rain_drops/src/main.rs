use std::io;
fn main() {
    let mut number = String::new();
    
    println!("Please enter a number: ");

    io::stdin().read_line(&mut number).expect("failed to read from stdin");

    let number : i32 = number.trim().parse().expect("invalid input");

    let mut word = String::new();

    if number % 3 == 0 {
        word.push_str("Pling ");
    }
    if number % 5 == 0 {
        word.push_str("Plang ");
    }
    if number % 7 == 0 {
        word.push_str("Plop ");
    }

    if number % 3 != 0 && number % 5 != 0 && number % 7 != 0 {
        println!("{}", number);
    }
    else{
        println!("{}", word);
    }
}
