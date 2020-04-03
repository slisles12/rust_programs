fn main() {
    let a = 6;

    if a < 10 {
        println!("a is less than 10!");
    }
    else {
        println!("a is greater than 10!");
    }

    if a % 4 == 0 {
        println!("a is divisible by 4!");
    }
    else if a % 3 == 0 {
        println!("a is divisible by 3!")
    }
    else if a % 2 == 0 {
        println!("a is divisible by 2!")
    }
    else{
        println!("a is not divisible by 4, 3, or 2!");
    }

}
