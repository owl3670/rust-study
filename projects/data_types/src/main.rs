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
    
    let _float64_0 = 2.0;
    let _float64_1 : f64 = 2.0;
    let _float32 : f32 = 2.0;
}
