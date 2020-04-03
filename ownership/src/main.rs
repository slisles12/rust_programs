fn main() {
    
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");

    let s2 = s1;

    //s1 is no longer valid or reacable
    println!("{}. world!", s2);

    let mut s3 = String::from("hello");

    takes_ownership(&mut s3);

    println!("{}", s3);

    let x = 5;

    makes_copy(x);

    fn takes_ownership(some_string: &mut String) {
        println!("{}", some_string);
        some_string.push_str("World!");
    }

    fn makes_copy(some_integer: i32){
        println!("{}", some_integer);
    }
}
