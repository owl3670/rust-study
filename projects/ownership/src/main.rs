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

    //=====================================================
    // Reference and borrowing

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");
    change1(&s);

    let mut s = String::from("hello");
    change2(&mut s);
    println!("{}", s);

    let mut _s = String::from("hello");
    let _r1 = &mut _s;
    // let r2 = &mut s; // error

    let mut _s = String::from("hello");
    {
        let _r1 = &mut _s;
    }
    let _r2 = &mut _s;

    let mut _s = String::from("hello");

    let _r1 = &_s; // no problem
    let _r2 = &_s; // no problem
    // let _r3 = &mut _s; // BIG PROBLEM

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    //=====================================================
    // Slices type

    let mut s = String::from("hello world");
    let word = first_word1(&s);
    s.clear();
    println!("The string is '{}', and first word's length is {}", s, word);

    let mut s = String::from("hello world");
    let word = first_word2(&s);
    // s.clear(); // error
    println!("The string is '{}', and first word is {}", s, word);

    let my_string = String::from("hello world");

    let word = first_word3(&my_string[0..6]);
    println!("The string is '{}', and first word is {}", my_string, word);
    let word = first_word3(&my_string[..]);
    println!("The string is '{}', and first word is {}", my_string, word);
    // to whole slices of `String`s
    let word = first_word3(&my_string);
    println!("The string is '{}', and first word is {}", my_string, word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word3(&my_string_literal[0..6]);
    println!("The string literal is '{}', and first word is {}", my_string_literal, word);
    let word = first_word3(&my_string_literal[..]);
    println!("The string literal is '{}', and first word is {}", my_string_literal, word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word3(my_string_literal);
    println!("The string literal is '{}', and first word is {}", my_string_literal, word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
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

fn calculate_length2(s: &String) -> usize {
    s.len()
}


fn change1(_some_string: &String) {
    // _some_string.push_str(", world"); // error
}


fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}, {}", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
