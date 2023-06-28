fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");

    change(&s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn change(_some_string: &String) {
    // _some_string.push_str(", world"); // error
}