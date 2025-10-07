extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Use Case: Identifier Validator ===\n");
    println!("This example shows how to validate programming language identifiers.");
    println!(
        "Valid identifiers: start with a letter, followed by zero or more letters or digits\n"
    );

    // Pattern for identifiers: letter followed by zero or more (letter or digit)
    // Simplified to (a|b|c) followed by zero or more (a|b|c|d) for demonstration
    let identifier_validator = regex!("(a|b|c)(a|b|c|d)*");

    println!("Pattern: \"(a|b|c)(a|b|c|d)*\"");
    println!("Meaning: Starts with a-c, followed by zero or more a-d\n");

    let test_cases = vec![
        ("a", true, "single letter"),
        ("abc", true, "multiple letters"),
        ("ad", true, "letter with digit-like char"),
        ("abcd", true, "letter with multiple chars"),
        ("cba", true, "different starting letter"),
        ("", false, "empty string"),
        ("d", false, "starts with invalid char"),
        ("1a", false, "starts with number-like char"),
    ];

    println!("Testing identifier validation:");
    for (input, expected, description) in test_cases {
        let result = identifier_validator.matches_exact(input);
        let status = if result == expected { "PASS" } else { "FAIL" };
        println!("[{}] '{}' -> {} ({})", status, input, result, description);
        assert_eq!(
            result, expected,
            "Failed for input '{}': {}",
            input, description
        );
    }

    println!("\nAll identifier validation tests passed!");
}
