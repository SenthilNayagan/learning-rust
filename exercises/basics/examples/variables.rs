// Variables - Every variable in Rust must have a type. It can either be inferred by the compiler or 
// explicitly specified by the developer.

fn main() {
    // let <variable_name>: <type> = <expression>;
    // We explicitly constrained the type of my_var to be u32
    let my_var: u32 = 42;

    // Type inference - If we don't specify the type of a variable, the compiler will try to infer it based on the 
    // context in which the variable is used
    // Inference limitations - The compiler sometimes needs a little help to infer the correct variable type based on its usage.
    // In those cases you'll get a compilation error and the compiler will ask you to provide an explicit type hint to 
    // disambiguate the situation.
    let a = 42;
    let b: u32 = a;

    // Initialization - We don't have to initialize a variable when we declare it.
    let c: u32;  // Valid variable declaration
    // However, we must initialize the variable before using it. The compiler will throw an error if we don't:
    let d: u32;
    let e = d + 1;  // Will throw a compilation error

    // Immutable variables (Default)
    let x = 5;
    println!("x = {x}");

    // Mutable variables
    let mut y = 10;
    println!("y = {y}");
    y += 1;
    println!("y after mutation = {y}");


}