fn main() {

    loop{
        println!("Again!");
        break;
    }


    let mut number = 3;

    println!("while loop start");
    
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }

    println!("for loop stat");

    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("{}", element);
    }
}
