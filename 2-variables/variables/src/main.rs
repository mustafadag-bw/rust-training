use std::io;

fn main() {
    let string_param = "string param";
    // characters are annotated with single quote
    let char_param = 'Z'; 
    let heart_eyed_cat = '😻';
    let t = true;
    let f: bool = false;
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, '*');
    let (x, y, z, w) = tup;
    let string_val = tup.3;
    // Arrays cannot change size in Rust. We should use vectors if the size is not determined.
    let array_a: [i32; 5] = [1, 2, 3, 4, 5];
    let array_b = [3;5]; // [3, 3, 3, 3, 3]
    let elem_a = array_a[0];
    let elem_b = array_b[0];
    println!("element 0 a: {elem_a}\nelement 0 b: {elem_b}");

    println!("Hello, world!\nstring : {string_param}, \nchar : {char_param}\nheart eyed cat: {heart_eyed_cat}");
    println!("The values of x, y, z, t: {x}, {y}, {z}, {string_val}.\nValue for t and f and t : {f}, {t}");

    let new_a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // If the index is greater than or equal to the length of array Rust panics. The memory management is on later parts.
    let element = new_a[index];

    println!("The value of the element at index {index} is: {element}");
}
