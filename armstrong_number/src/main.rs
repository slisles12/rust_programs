use std::io;
fn main() {
    
    let mut number = String::new();

    println!("Please input a number: ");

    io::stdin().read_line(&mut number).expect("unable to read input");

    let is_true = armstrong_number(&number);

    if is_true {
        print!("The number is an armstrong number");
    }
    else { 
        print!("The number is not an armstrong number");
    }

}

fn armstrong_number(x : &str) -> bool {

    let mut sum = 0;

    let expected : u32 = x.trim().parse().expect("invalid input");

    for (i, c) in x.chars().enumerate() {
        if i < x.chars().count() - 2 { 
        
            let t = c as u32 - 48;
            sum = sum + t.pow((x.chars().count() - 2) as u32);
        }
    }

    if sum == expected {
        return true;
    }

    println!("{}", sum);
    
    return false;
}