// Shadowing means declaring a new variable with the same name as a previous variable, which hides (shadows) 
// the earlier variable. The old variable still exists internally, but the new one takes its place in the current scope.

// Shadowing vs. `mut`:
// Rust allows changing values in two ways:
// 1. Using `mut` (mutable variable - same variable and value is updated)
// 2. Using shadowing (new variable created and old variable hidden)

// Why Rust supports shadowing:
// Shadowing is useful when value of one type into other
// Example: let space = " "; let spaces = spaces.len();
// First spaces → string. Second spaces → integer. 
// This would not work with mut, because type cannot change.

// Why shadowing has zero runtime cost:
// - Resolved during compilation
// - Variable names disappear early
// - Each binding becomes a separate internal variable
// - Optimizations remove unnecessary temporaries
//
// Shadowing is purely a compile-time name resolution feature.

// Summary:
// Shadowing in Rust means:
// - Declaring a new variable with the same name
// - The new variable hides the previous one
// - It allows type changes and transformations
// - Different from mut, which modifies the same variable

fn main() {
    let z = 10;  // A variable z is created with value 10
    
    // This creates a new variable also named z.
    // Rust evaluates the right side first using the previous z i.e. z * 2 = 10 * 2 = 20.
    // Then a new variable z = 20 replaces (shadows) the previous one.
    // The earlier z = 10 is no longer accessible.
    let z = z * 2;

    println!("z shadowed = {z}");
}