fn main() {

    let user1 = User {
        username: String::from("slisles"),
        email: String::from("example@xyz.com"),
        active: true,
        sign_in_count: 100,
    };

    println!();
    println!("user1's email address {}", user1.email);
    println!("user1's username {}", user1.username);
    println!("user1 is active: {}", user1.active);
    println!("user1 has signed in {} many times", user1.sign_in_count);

    let user2 = User {
        email:String::from("spencerlisle12"),
        username:String::from("lislees"),
        ..user1
    };

    println!();
    println!("user2's email address {}", user2.email);
    println!("user2's username {}", user2.username);
    println!("user2 is active: {}", user2.active);
    println!("user2 has signed in {} many times", user2.sign_in_count);

    let s1 = String::from("bob");
    let s2 = String::from("bob@aol.com");
    let user3 = build_user(s1, s2);
    println!();
    println!("user3's email address {}", user3.email);
    println!("user3's username {}", user3.username);
    println!("user3 is active: {}", user3.active);
    println!("user3 has signed in {} many times", user3.sign_in_count);


}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user (email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
