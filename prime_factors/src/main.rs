use std::io;
fn main() {

    let mut number = String::new();
    
    println!("Please enter a number: ");

    io::stdin().read_line(&mut number).expect("failed to read from stdin");

    let number : u64= number.trim().parse().expect("invalid input");

    print!("The prime factors of {} are", number);

    let v = prime_factors(number);

    for x in v {
        print!(" {} ", x);
    }


}

fn prime_factors(mut x: u64) -> Vec<u64> {

    let mut v: Vec<u64> = Vec::new();

    loop {

        let mut was_pushed = false;

        for i in 2..x+1 {
            if x % i == 0 {
                v.push(i);
                x = x/i;
                was_pushed = true;
                break;
           }
        }

        if was_pushed == false {
            break;
        }
    }
     
    return v;

}