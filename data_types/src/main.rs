fn main() {
    // integer types

    // signed capacity -(2^(n-1)) to 2^(n-1) inclusive
    // unsigned capacity 0 to (2^n) - 1

    // 8 bits
    let 8_bits: i8
    let 8_bits_u: u8

    // 16 bits
    let 16_bits: i16
    let 16_bits_u: u16

    // 32 bits
    let 32_bits: i32
    let 32_bits_u: u32

    // 64 bits
    let 64_bits: i64
    let 64_bits_u: u64

    // 128 bits
    let 128_bits: i128
    let 128_bits_u: u128

    // arch bits (depend on the architecture of the computer your program is running on)
    let arch_bits: isize
    let arch_bits_u: usize

    // floating point types
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // boolean types
    let t = true;

    let f: bool = false; // with explicit type annotation

    // character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1); // with explicit type annotation

    // destructuring tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // direct access tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // array type
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation and length

    let a = [3; 5]; // array with the same value for all elements, ==  let a = [3, 3, 3, 3, 3]

    // accessing array elements
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
