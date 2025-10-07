extern crate gregex;
use gregex::*;

fn main() {
    // Test string concatenation
    let runner = regex!("abc");
    println!("Testing \"abc\":");
    assert_eq!(runner.matches_exact("abc"), true);
    assert_eq!(runner.matches_exact("ab"), false);
    assert_eq!(runner.matches_exact("abcd"), false);
    println!("✓ String concatenation works!");

    // Test string star
    let runner2 = regex!("(ab)*");
    println!("\nTesting \"(ab)*\":");
    assert_eq!(runner2.matches_exact(""), true);
    assert_eq!(runner2.matches_exact("ab"), true);
    assert_eq!(runner2.matches_exact("abab"), true);
    assert_eq!(runner2.matches_exact("aba"), false);
    println!("✓ String star works!");

    // Test string plus
    let runner3 = regex!("(ab)+");
    println!("\nTesting \"(ab)+\":");
    assert_eq!(runner3.matches_exact("ab"), true);
    assert_eq!(runner3.matches_exact("abab"), true);
    assert_eq!(runner3.matches_exact(""), false);
    println!("✓ String plus works!");

    // Test string question
    let runner4 = regex!("(ab)?");
    println!("\nTesting \"(ab)?\":");
    assert_eq!(runner4.matches_exact(""), true);
    assert_eq!(runner4.matches_exact("ab"), true);
    assert_eq!(runner4.matches_exact("abab"), false);
    println!("✓ String question works!");

    println!("\n🎉 All string literal tests passed!");
}
