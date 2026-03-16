// Ownership is a set of rules that govern how a Rust program manages memory
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
// The data types are of a known size, can be stored on the stack and popped off the stack when their scope is over.

fn main() {
    let a: String = String::from("Hello");
    println!("{} John!", a);

    // let b = a.clone();
    // When we assign a to b, the ownership moves. This means only b can use the value now, because a is no longer valid.
    let b = a;
    println!("{} John! from variable a", a);
    println!("{} John! from variable b", b);

    // But simple types like numbers, characters and booleans are copied, not moved.
    // This means we can still use the original variable after assigning it to another
    // Here, a is copied into b, not moved, so we can still use b
    let a = 5;
    let b = a;
    println!("a = {}", a);  // Works
    println!("b = {}", b);  // Works
}