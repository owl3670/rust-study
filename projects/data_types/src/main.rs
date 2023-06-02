fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // It's okay
    // let guess = "42".parse().expect("Not a number!"); // error (type annotation needed)

    // Scalar Types
    // Integer Types

    let int8: i8 = -10; 
    let uint8: u8 = 10; 

    let int16: i16 = -10;
    let uint16: u16 = 10;

    let int32: i32 = -10;
    let uint32: u32 = 10;

    let int64: i64 = -10;
    let uint64: u64 = 10;

    let int128: i128 = -10;
    let uint128: u128 = 10;

    let arch: isize = -10; // depends on the architecture of the computer
    let uarch: usize = 10;

    let dec: i32 = 100_000;
    let hex: i32 = 0xff;
    let octal: i32 = 0o77;
    let binary: i32 = 0b1111_0000;
    let byte: u8 = b'A'; // u8 only
}
