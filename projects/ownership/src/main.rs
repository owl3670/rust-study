fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s 가 scope 에 들어옴

    takes_ownership(s); // s 의 값이 함수로 이동함. 따라서 이 시점에서 s 는 더이상 유효하지 않음.
    // println!("{}", s); // error!

    let x = 5; // x 가 scope 에 들어옴

    makes_copy(x); // x 는 함수로 이동되지만, i32 는 Copy 이므로 이후에도 x 를 계속 사용할 수 있음.
    println!("{}", x); // ok!

    let s1 = gives_ownership();         // gives_ownership 함수가 반환하는 값을 s1 에 대입함

    let s2 = String::from("hello");     // s2 가 scope 에 들어옴

    let s3 = takes_and_gives_back(s2);  // s2 를 takes_and_gives_back 함수로 이동하고, 반환값을 s3 에 대입함

    println!("s1 = {}, s3 = {}", s1, s3); // s1, s3 는 여기서 유효함

    // println!("{}", s2); // error!

    let s1 = String::from("hello");
    let (_s2, _len) = calculate_length(s1);
}

fn takes_ownership(some_string: String) { // some_string 변수가 scope 에 들어옴
    println!("{}", some_string);
} // 여기서 some_string 이 scope 밖으로 벗어나고 `drop` 이 호출됨. some_string 의 backing memory 가 해제됨.

fn makes_copy(some_integer: i32) { // some_integer 변수가 scope 에 들어옴
    println!("{}", some_integer);
} // 여기서 some_integer 가 scope 밖으로 벗어나지만, 별다른 일이 발생하지 않음.

fn gives_ownership() -> String {             // gives_ownership 함수가 반환하는 값이 호출한 함수로 이동됨
    let some_string = String::from("hello"); // some_string 변수가 scope 에 들어옴

    some_string                              // some_string 이 반환되고, 호출한 함수로 이동됨
}

// takes_and_gives_back 함수는 String 을 하나 받아서 다른 하나를 반환함
fn takes_and_gives_back(a_string: String) -> String { // a_string 변수가 scope 에 들어옴
    a_string  // a_string 이 반환되고, 호출한 함수로 이동됨
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 메소드는 String 의 길이를 반환함

    (s, length)
}