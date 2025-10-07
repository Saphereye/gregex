extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Kleene Star (*) Example ===\n");
    println!("This example demonstrates the Kleene star operator (*).");
    println!("The star operator matches zero or more occurrences.\n");

    // Create a pattern that matches zero or more 'a's
    let pattern = regex!("a*");

    println!("Pattern: \"a*\" (zero or more 'a's)\n");

    // Test various repetitions
    println!("Testing matches:");
    assert!(pattern.matches_exact(""));
    println!("  \"\" (empty) matches: true");

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
