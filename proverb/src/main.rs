fn main() {
    let input = vec!["nail", "shoe", "horse"];

    let mut word = build_proverb(&input);

    println!("{}", word);
}

fn build_proverb(list: &[&str]) -> String {

    let mut word = String::new();

    for x in 0..(list.len()-1) {
        word.push_str(&format!("For the want of a {} the {} was lost \n", list[x], list[x+1]));
    }

    word.push_str(&format!("And all for the want of {}", list[0]));


    return word;

}