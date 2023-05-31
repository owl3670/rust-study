const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // cannot assign twice to immutable variable
    
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}
