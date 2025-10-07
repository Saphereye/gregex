extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Alternation (OR) Example ===\n");
    println!("This example demonstrates the alternation operator (|).");

    // Create a pattern that matches "a" OR "b" OR "c"
    let pattern = regex!("a|b|c");

    println!("Pattern: \"a|b|c\" (matches 'a', 'b', or 'c')\n");

    // Test each alternative
    println!("Testing matches:");
    assert!(pattern.matches_exact("a"));
    println!("  \"a\" matches: true");

    assert!(pattern.matches_exact("b"));
    println!("  \"b\" matches: true");

    assert!(pattern.matches_exact("c"));
    println!("  \"c\" matches: true");

    assert!(!pattern.matches_exact("d"));
    println!("  \"d\" matches: false");

    assert!(!pattern.matches_exact("ab"));
    println!("  \"ab\" matches: false (too long)");

    println!("\nAll tests passed!");
}
