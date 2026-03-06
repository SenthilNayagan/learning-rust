// Rust data types - Scalar types
// Keep in mind that Rust is a statically typed language, which means that it must know the types
// of all variables at compile time. The compiler can usually infer what type we want to use based on 
// the value and how we use it.

fn main() {
    // Scalar types. It represents a single value.
    // Rust has four primary scalar types:
    // - Integers
    // - Floating-point numbers
    // - Booleans
    // - Characters

    // Integers
    let my_num: u16 = 10;
    println!("my_num = {my_num}");

    // Floating-point numbers
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // Note: The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is 
    // capable of more precision. 
    // All floating-point types are signed.
    let my_float_num = 2.0;  // default to f64
    println!("my_float_num = {my_float_num}");

    let y: f32 = 1.0;  // f32
    println!("y = {y}");

    // Boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {t}, f = {f}");

    // Character type - specify with single quotation marks, as opposed to string literals, which 
    // use double quotation marks.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c = {c}, z = {z}, heart_eyed_cat = {heart_eyed_cat}");
}