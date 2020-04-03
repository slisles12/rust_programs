use std::io;
fn main() {
    let mut nth = String::new();
    
    println!("Please enter the nth prime you would like to find: ");

    io::stdin().read_line(&mut nth).expect("failed to read from stdin");

    let nth : i32 = nth.trim().parse().expect("invalid input");
    let mut num_primes = 0;
    let mut next_prime = 1;

    while num_primes != nth+1{
        next_prime = next_prime + 1;
        if is_prime(next_prime as f64) {
            num_primes = num_primes + 1;
        }
    }

    println!("The {}th prime is {}", nth, next_prime);
}

fn is_prime(x: f64) -> bool{
    let x = x.round() as i32;

    for i in 2..x {
        if x % i == 0{
            return false;
        }
    }
    
    return true;
}
