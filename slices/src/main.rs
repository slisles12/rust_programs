fn main() {
    let s = String::from("Hello, World!");

    let hello = &s[0..5];

    println!("{}", hello);

    let a = [1,2,3,4,5];

    let slice = &a[0..3];

    for element in slice.iter() {
        println!("{}", element);
    }
}
