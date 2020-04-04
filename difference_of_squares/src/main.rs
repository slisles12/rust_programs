use std::io;
fn main() {
    let mut number = String::new();
    
    println!("Please enter a number: ");

    io::stdin().read_line(&mut number).expect("failed to read from stdin");

    let number : i32 = number.trim().parse().expect("invalid input");

    println!("The difference squares with {} is {}", number, difference_of_squares(number));
}


fn difference_of_squares(x: i32) -> i32{
    return (square_of_sum(x) - sum_of_square(x));
}

fn square_of_sum(x: i32) -> i32 {

    let mut calc = 0;

    for i in 1..x+1 {
        calc = calc + i;
    }

    return calc * calc;


}

fn sum_of_square(x: i32) -> i32 {

    let mut calc = 0;

    for i in 1..x+1 {
        calc = calc + (i * i);
    }

    return calc;
    
}