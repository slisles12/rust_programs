use std::io;
fn main() {
    let mut text = String::new();
    
    println!("Speak to Bob: ");

    io::stdin().read_line(&mut text).expect("failed to read from stdin");

    let reply = reply(&text);

    println!("Bob's reply: {}", reply);
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match message{
        m if m.trim().is_empty() => "Fine. Be that way!",
        m if m.ends_with('?') && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with('?') => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever",
    }
}

fn is_yelling(message: &str) -> bool {
    &(message.to_uppercase()) == message && message.chars().any(|x| x.is_alphabetic())
}
