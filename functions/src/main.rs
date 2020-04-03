fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "Ovtober", "November", "December"];

    println!("First month = {}", months[0]);
    
    another_function();

    another_function_two(5);

    println!("The sum of 4 and 8 is {}", sum(4, 8));
}

fn another_function(){
    println!("Another funciton.");
}

fn another_function_two(x:i32){
    println!("The value of x = {}", x);
}

fn sum(x:i32, y:i32) -> i32{
    return x + y;
}