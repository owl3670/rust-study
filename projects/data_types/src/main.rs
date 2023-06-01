fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // It's okay
    // let guess = "42".parse().expect("Not a number!"); // error (type annotation needed)
}
