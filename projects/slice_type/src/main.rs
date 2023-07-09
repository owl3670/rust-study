fn main() {
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