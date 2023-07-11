struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        username: String::from("testname"),
        email: String::from("testemail"),
        sign_in_count: 1,
        active: true,
    };

    println!("user1's email: {}", user1.email);

    let mut user1 = User {
        username: String::from("testname"),
        email: String::from("testemail"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("testemail2");
    println!("user1's email: {}", user1.email);

    let user1 = build_user(String::from("testemail3"), String::from("testname3"));
    println!("user1's email: {}", user1.email);
}
