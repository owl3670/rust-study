fn main() {
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
    
}
