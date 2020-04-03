fn main() {
    let tup: (i32, f64, u8) = (500, 3.5, 1);
    
    let (x, y, z) = tup;

    println!("The values of x, y, z are equal to {}, {}, {}" , x, y, z);

    let a = tup.0;
    let b = tup.1;

    println!("The values of a, b are {}, {}", a, b);

}


