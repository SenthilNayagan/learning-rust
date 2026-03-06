// Like immutable variables, constants are values that are bound to a name and are not 
// allowed to change, but there are a few differences between constants and variables.

// First, we aren’t allowed to use mut with constants. Constants aren’t just immutable by default—
// they’re always immutable.

// We declare constants using the `const` keyword instead of the let keyword, and the type of the 
// value must be annotated.

// Rust’s naming convention for constants is to use all uppercase with underscores between words.

// Note: Constants may be set only to a constant expression, not the result of a value that could 
// only be computed at runtime.

fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {MAX_POINTS}");

    // Constant set to a constant expression
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS = {THREE_HOURS_IN_SECONDS}");
}