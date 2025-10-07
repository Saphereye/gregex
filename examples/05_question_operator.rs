extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Question (?) Operator Example ===\n");
    println!("This example demonstrates the question operator (?).");
    println!("The question operator matches zero or one occurrence.\n");

    // Create a pattern that matches zero or one 'a'
    let pattern = regex!("a?");

    println!("Pattern: \"a?\" (zero or one 'a')\n");

    // Test various cases
    println!("Testing matches:");
    assert!(pattern.matches_exact(""));
    println!("  \"\" (empty) matches: true");

    assert!(pattern.matches_exact("a"));
    println!("  \"a\" matches: true");

    assert!(!pattern.matches_exact("aa"));
    println!("  \"aa\" matches: false (too many)");

    assert!(!pattern.matches_exact("b"));
    println!("  \"b\" matches: false");

    println!("\nAll tests passed!");
}
