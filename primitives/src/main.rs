use std::fmt;
use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let (a, b, c, d) = (matrix.0, matrix.1, matrix.2, matrix.3);
    Matrix(a, c, b, d)
}


// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let default_float = 3.0;
    let default_integer = 7;

    let mut mutable = 12;
    println!("{}", mutable);
    mutable = 21;

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let my_tuple = (123, "foo", "bar", 578);

    println!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", logical, a_float, default_float, default_integer, mutable, my_array, my_tuple);

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);

    println!("This is a big number: {}", 1_000_001);

    // Tuples
    println!("\n\nTuples stuff!");
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1, 0.2, 'a', true);
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple 6th value: {}", long_tuple.6);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
        
    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Reversed pair is {:?}", reverse(pair));

    println!("One element tuple: {:?}", (10u32,));

    let tuple = (1, "yo", "hi", 2);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    // Arrays and Slices
    println!("\n\nArrays and Slices!");
    let xs: [i32; 5] = [1, 2,3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    println!("Number of elements in the array: {}", xs.len());
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    analyze_slice(&ys[1 .. 4]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // oob
    //println!("{}", xs[5]);

}

