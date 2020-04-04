use std::io;
fn main() {
    
    let mut word = String::new();

    println!("Please input a word: ");

    io::stdin().read_line(&mut word).expect("unable to read input");

    word = word.replace("\n", "");

    println!("The reverse is : ");
    print!("{}", string_reverse(&word));
}

fn string_reverse(word: &str) -> String {

    let mut new_word = String::new();

    for c in word.chars().rev(){
        new_word.push(c);
    }

    return new_word;

}