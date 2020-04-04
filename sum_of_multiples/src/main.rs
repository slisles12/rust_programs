fn main() {

    let returned = sum_of_multiples(20, &[3, 5]);
    println!("{}" , returned);

}

fn sum_of_multiples(x : u32, factors: &[u32]) -> u32 {

    let mut sum = 0;

    for i in 1..x {
        for j in factors {
            if i % j == 0 {
                sum = sum + i;
                break;
            }
        }
    }

    return sum;

}