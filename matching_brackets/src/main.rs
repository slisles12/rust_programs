use std::io;
fn main() {
    let mut text = String::new();
    
    println!("Enter your string of brackets: ");

    io::stdin().read_line(&mut text).expect("failed to read from stdin");

    let new_text = text.trim();

    let reply = does_match(&new_text);

    if reply {
        println!("The brackets match");
    }
    else {
        println!("The brackets do not match");
    }


}

fn is_open_bracket(c: char) -> bool {
    c == '{' || c == '[' || c == '('
}

fn is_closed_bracket(c: char) -> bool {
    c == '}' || c == ']' || c == ')'
}

fn is_correct(c: char, t: Option<char>) -> bool {

    let x = t.unwrap();

    if x == '{' && c == '}' {
        return true;
    }
    if x == '[' && c == ']' {
        return true;
    }
    if x == '(' && c == ')' {
        return true;
    }
    false
}

fn does_match(text: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in text.chars() {
        if is_open_bracket(c){
            stack.push(c);
        }
        else{
            break;
        }
    }

    let text = &text[text.len()/2..];

    for c in text.chars(){
        if is_closed_bracket(c){
            if !is_correct(c, stack.pop()) {
                return false
            }
        }
        else{
            break;
        }
    }

    if stack.len() == 0 {
        return true
    }

    false
}
