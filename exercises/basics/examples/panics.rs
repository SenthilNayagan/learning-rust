// Panics
// $env:RUST_BACKTRACE=1; cargo run -p basics --example panics

fn main() {
    // panic!("Something went terribly wrong!");

    // let numbers = vec![10, 20, 30];
    // println!("{}", numbers[4]);

    // Unwrapping a None
    // unwrap() is a convenience method used to extract the value inside 
    // Option or Result. If the value is not present, it panics and stops the program
    let x: Option<i32> = None;
    println!("{}", x.unwrap());  // panics: called unwrap on None
}