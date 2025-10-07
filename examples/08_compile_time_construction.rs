extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Compile-Time NFA Construction Example ===\n");
    println!("This example verifies that regex patterns are constructed at compile-time.");
    println!(
        "Use 'cargo expand --example 08_compile_time_construction' to see the expanded code.\n"
    );

    // All of these patterns are compiled to NFA at compile-time,
    // resulting in zero runtime overhead for NFA construction

    println!("Testing character literal:");
    let char_pattern = regex!('a');
    assert!(char_pattern.matches_exact("a"));
    println!("  regex!('a') works\n");

    println!("Testing simple string:");
    let simple_pattern = regex!("abc");
    assert!(simple_pattern.matches_exact("abc"));
    println!("  regex!(\"abc\") works\n");

    println!("Testing complex pattern:");
    let complex_pattern = regex!("(a|b)+c?");
    assert!(complex_pattern.matches_exact("abc"));
    assert!(complex_pattern.matches_exact("ab"));
    assert!(complex_pattern.matches_exact("bac"));
    println!("  regex!(\"(a|b)+c?\") works\n");

    println!("All patterns are constructed at compile-time!");
    println!("\nNote: The NFA is embedded directly in the binary,");
    println!("eliminating all runtime regex parsing overhead.");
}
