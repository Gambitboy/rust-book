fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types
    let _flt: f64; // Float 64bit
    let _int: i32; // integer 32bit signed
    let _arch_int: isize; // Architecture dependant
    let _boolean: bool; // boolean
    let _character: char; // chararacter

    // Integers
    let _thousand: u32 = 1_000;
    let _large: u8 = 255;

    // Floats
    let _x = 2.0;
    let _y: f32 = 3.0;

    // Operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;
    let _remainder = 43 % 5;

    // Compound Types
    // Tuple - Fixed
    let tup: (i32, f64, u8, bool, char) = (500, 64.1, 1, false, 'A');
    let (v, w, x, y, z) = tup;
    println!("The tuples are: {v}, {w}, {x}, {y}, {z}");
    let v = tup.0;
    let w = tup.1;
    let x = tup.2;
    let y = tup.3;
    let z = tup.4;
    println!("The tuples are: {v}, {w}, {x}, {y}, {z}");

    // Array - Stack allocation, Fixed
    let _some_months = ["January", "February", "March"];
    let _numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let _same = [3; 5];
    println!("Same array: {:?}", _same);

    let _first = _numbers[0];
    let _second = _numbers[1];
}
