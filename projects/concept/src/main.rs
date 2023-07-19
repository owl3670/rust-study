use std::io;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // =====================================================
    // Variables and Mutability

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

    // =====================================================
    // Data Types

    let _guess: u32 = "42".parse().expect("Not a number!"); // It's okay
    // let guess = "42".parse().expect("Not a number!"); // error (type annotation needed)

    // Scalar Types
    // Integer Types

    let _int8: i8 = -10;
    let _uint8: u8 = 10;

    let _int16: i16 = -10;
    let _uint16: u16 = 10;

    let _int32: i32 = -10;
    let _uint32: u32 = 10;

    let _int64: i64 = -10;
    let _uint64: u64 = 10;

    let _int128: i128 = -10;
    let _uint128: u128 = 10;

    let _arch: isize = -10; // depends on the architecture of the computer
    let _uarch: usize = 10;

    let _dec: i32 = 100_000;
    let _hex: i32 = 0xff;
    let _octal: i32 = 0o77;
    let _binary: i32 = 0b1111_0000;
    let _byte: u8 = b'A'; // u8 only

    // Floating-Point Types

    let _float64_0 = 2.0;
    let _float64_1 : f64 = 2.0;
    let _float32 : f32 = 2.0;

    // Numeric Operations

    let sum = 5 + 10;
    println!("5 + 10 = {sum}");
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");
    let product = 4 * 30;
    println!("4 * 30 = {product}");
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3;
    println!("-5 / 3 = {truncated}");
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

    // Boolean Type

    let _true = true;
    let _false: bool = false;

    // Character Type

    let _c = 'z';
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';

    // Compound Types
    // Tuple Type

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The tuple is: {:?}", tup);
    println!("The 0th index element of the tuple is: {}", tup.0);
    println!("The 1th index element of the tuple is: {}", tup.1);
    println!("The 2th index element of the tuple is: {}", tup.2);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // Array Type

    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("{:?}", a);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    println!("a[0] = {}", a[0]);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // =====================================================
    // Functions

    println!("Hello, world!");

    another_function();
    another_param_function(30);
    add(3, 5);

    let x = 6; // Statement
    println!("The value of x is: {x}");
    let y = {       // Expression
        let z = 3;
        x + z
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

    // =====================================================
    // Control Flow

    // if expressions
    let number = 6;

    if number % 5 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six"}; // Error occurred
    println!("The value of number is: {number}");

    // Repetition with Loops
    let mut counter = 0;
    let result = loop {
        println!("again!");
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("End of while");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev(){
        println!("{number}");
    }
}

fn another_function() {
    println!("Another function.");
}

fn another_param_function(x: i32) {
    println!("The value of x is: {x}");
}

fn add(a: i32, b: i32){
    println!("{a} + {b} = {}", a+b);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32{
    //x + 1; // Error occured
    x + 1
}
