fn main() {
    let mut s = String::from("hello world");
    let word = first_word1(&s);
    s.clear();
    println!("The string is '{}', and first word's length is {}", s, word);

    let mut s = String::from("hello world");
    let word = first_word2(&s);
    // s.clear(); // error
    println!("The string is '{}', and first word is {}", s, word);
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