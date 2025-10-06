extern crate gregex;
use gregex::*;

fn main() {
    // Test character literal (compile-time NFA)
    let runner1 = regex!('a');
    assert!(runner1.is_match("a"));
    assert!(runner1.is_match("bac"));
    assert!(!runner1.is_match("bc"));
    println!("✓ Character literal regex works!");

    // Test string literal (compile-time NFA)
    let runner2 = regex!("abc");
    assert!(runner2.is_match("abc"));
    assert!(runner2.is_match("xabcy"));
    assert!(!runner2.is_match("ab"));
    println!("✓ String literal regex works!");

    // Test complex pattern (compile-time NFA)
    let runner3 = regex!("a+b*");
    assert!(runner3.is_match("a"));
    assert!(runner3.is_match("ab"));
    assert!(runner3.is_match("aabbb"));
    assert!(!runner3.is_match("b"));
    println!("✓ Complex pattern regex works!");

    println!("\n🎉 All compile-time NFA construction tests passed!");
}
