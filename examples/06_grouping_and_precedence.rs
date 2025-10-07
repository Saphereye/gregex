extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Grouping and Precedence Example ===\n");
    println!("This example demonstrates how parentheses control operator precedence.\n");

    // Pattern: (ab)+ means "ab" repeated one or more times
    let pattern1 = regex!("(ab)+");
    println!("Pattern 1: \"(ab)+\" (one or more \"ab\" sequences)\n");

    println!("Testing pattern 1:");
    assert!(pattern1.matches_exact("ab"));
    println!("  \"ab\" matches: true");

    assert!(pattern1.matches_exact("abab"));
    println!("  \"abab\" matches: true");

    assert!(pattern1.matches_exact("ababab"));
    println!("  \"ababab\" matches: true");

    assert!(!pattern1.matches_exact("aba"));
    println!("  \"aba\" matches: false (incomplete sequence)");

    assert!(!pattern1.matches_exact(""));
    println!("  \"\" matches: false (requires at least one)\n");

    // Pattern: (a|b)* means any combination of 'a' and 'b', zero or more times
    let pattern2 = regex!("(a|b)*");
    println!("Pattern 2: \"(a|b)*\" (any combination of 'a' and 'b')\n");

    println!("Testing pattern 2:");
    assert!(pattern2.matches_exact(""));
    println!("  \"\" matches: true");

    assert!(pattern2.matches_exact("a"));
    println!("  \"a\" matches: true");

    assert!(pattern2.matches_exact("b"));
    println!("  \"b\" matches: true");

    assert!(pattern2.matches_exact("ab"));
    println!("  \"ab\" matches: true");

    assert!(pattern2.matches_exact("ba"));
    println!("  \"ba\" matches: true");

    assert!(pattern2.matches_exact("aabbba"));
    println!("  \"aabbba\" matches: true");

    assert!(!pattern2.matches_exact("c"));
    println!("  \"c\" matches: false");

    println!("\nAll tests passed!");
}
