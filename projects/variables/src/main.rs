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

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "  ";
    let spaces = spaces.len();

    // let mut spaces = "  ";
    // spaces = spaces.len();
    // mismatched types
}
