fn main() {
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
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn change1(_some_string: &String) {
    // _some_string.push_str(", world"); // error
}


fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}