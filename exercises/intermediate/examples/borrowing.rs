// Borrowing is a way to use a value without taking ownership of it.
// Rust lets us do this using a reference - this is called borrowing
// We create a reference using the & symbol
// Borrowing helps you reuse values safely, without giving them away
// It lets you use values without taking ownership
// It avoids cloning, which can be slow for large data
// It makes your programs safer and faster

fn main() {
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);

    // Mutable references
    // Note: We can only have one mutable reference to a value at a time!
    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");

    println!("{}", name_ref);  // John Doe
}