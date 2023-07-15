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

fn build_user1(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

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

    let user1 = build_user1(String::from("testemail4"), String::from("testname4"));
    println!("user1's email: {}", user1.email);

    let username = String::from("testname5");
    let email = String::from("testemail5");
    let user1 = User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    };
    println!("user1's email: {}", user1.email);

    let user1 = User {
        email: String::from("testemail"),
        username: String::from("testname"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("testemail2"),
        username: user1.username,
        ..user1
    };

    // println!("user1's email: {}", user1.username); // Error occured

    let user1 = User {
        email: String::from("testemail"),
        username: String::from("testname"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("testemail2"),
        username: String::from("testname2"),
        ..user1
    };

    println!("user1's email: {}", user1.username);

    let mut black = Color(0, 0, 0);
    let origin = Point(1, 0, 0);

    // black = origin; // Error occured
    black.0 = origin.0;
    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);
}
