extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Plus (+) Operator Example ===\n");
    println!("This example demonstrates the plus operator (+).");
    println!("The plus operator matches one or more occurrences.\n");

    // Create a pattern that matches one or more 'a's
    let pattern = regex!("a+");

    println!("Pattern: \"a+\" (one or more 'a's)\n");

    // Test various repetitions
    println!("Testing matches:");
    assert!(!pattern.matches_exact(""));
    println!("  \"\" (empty) matches: false (requires at least one)");

    assert!(pattern.matches_exact("a"));
    println!("  \"a\" matches: true");

    assert!(pattern.matches_exact("aa"));
    println!("  \"aa\" matches: true");

    assert!(pattern.matches_exact("aaa"));
    println!("  \"aaa\" matches: true");

    assert!(!pattern.matches_exact("b"));
    println!("  \"b\" matches: false");

    assert!(!pattern.matches_exact("ab"));
    println!("  \"ab\" matches: false");

    println!("\nAll tests passed!");
}
