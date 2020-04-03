use std::fs::File;

fn main() {
    
    //panic!("Crash and burn");

    //preform backtrace = $Env:RUST_BACKTRACE = 1
    
    let f = File::open("hello.txt").expect("we don't have the file yet!");

    // let foo = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("File was not found!");
    //     },
    // };

}
