extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Basic Concatenation Example ===\n");
    println!("This example demonstrates simple character concatenation.");

    // Create a pattern that matches the exact string "abc"
    let pattern = regex!("abc");

    println!("Pattern: \"abc\"\n");

    // Test exact matches
    println!("Testing exact matches:");
    assert_eq!(pattern.matches_exact("abc"), true);
    println!("  \"abc\" matches: true");

    assert_eq!(pattern.matches_exact("ab"), false);
    println!("  \"ab\" matches: false (too short)");

    assert_eq!(pattern.matches_exact("abcd"), false);
    println!("  \"abcd\" matches: false (too long)");

    // Test substring matching
    println!("\nTesting substring matching:");
    assert!(pattern.is_match("abc"));
    println!("  is_match(\"abc\"): true");

    assert!(pattern.is_match("prefix_abc_suffix"));
    println!("  is_match(\"prefix_abc_suffix\"): true");

    assert!(!pattern.is_match("ab"));
    println!("  is_match(\"ab\"): false");

    println!("\nAll tests passed!");
}
