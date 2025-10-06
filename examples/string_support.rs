extern crate gregex;
use gregex::*;

fn main() {
    // Test string support in dot!
    let runner = regex!(dot!("abc"));
    println!("Testing dot!(\"abc\"):");
    assert_eq!(runner.run("abc"), true);
    assert_eq!(runner.run("ab"), false);
    assert_eq!(runner.run("abcd"), false);
    println!("✓ String concatenation works!");

    // Test string support in star!
    let runner2 = regex!(star!("ab"));
    println!("\nTesting star!(\"ab\"):");
    assert_eq!(runner2.run(""), true);
    assert_eq!(runner2.run("ab"), true);
    assert_eq!(runner2.run("abab"), true);
    assert_eq!(runner2.run("aba"), false);
    println!("✓ String star works!");

    // Test string support in plus!
    let runner3 = regex!(plus!("ab"));
    println!("\nTesting plus!(\"ab\"):");
    assert_eq!(runner3.run("ab"), true);
    assert_eq!(runner3.run("abab"), true);
    assert_eq!(runner3.run(""), false);
    println!("✓ String plus works!");

    // Test string support in question!
    let runner4 = regex!(question!("ab"));
    println!("\nTesting question!(\"ab\"):");
    assert_eq!(runner4.run(""), true);
    assert_eq!(runner4.run("ab"), true);
    assert_eq!(runner4.run("abab"), false);
    println!("✓ String question works!");

    println!("\n🎉 All string literal tests passed!");
}
