extern crate gregex;
use gregex::*;

fn main() {
    // Real-world example: Simple identifier validation
    // Valid identifiers: start with a letter, followed by zero or more letters or digits
    // Pattern: letter(letter|digit)*

    println!("=== Identifier Validator ===\n");

    // Pattern for lowercase identifiers: a-z followed by zero or more a-z or 0-9
    // Simplified to just 'a' followed by zero or more 'a' or 'b' for demonstration
    let identifier_validator = regex!(dot!(
        or!('a', 'b', 'c'),             // First character must be a letter
        star!(or!('a', 'b', 'c', 'd'))  // Followed by zero or more letters/digits
    ));

    let test_cases = vec![
        ("a", true, "single letter"),
        ("abc", true, "multiple letters"),
        ("ad", true, "letter with digit"),
        ("abcd", true, "letter with multiple chars"),
        ("", false, "empty string"),
        ("d", false, "starts with digit"),
        ("1a", false, "starts with number"),
    ];

    println!("Testing identifier validation:");
    for (input, expected, description) in test_cases {
        let result = identifier_validator.run(input);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} '{}' -> {} ({})", status, input, result, description);
        assert_eq!(
            result, expected,
            "Failed for input '{}': {}",
            input, description
        );
    }

    println!("\n=== URL Path Matcher ===\n");

    // Pattern for matching paths like: /a, /aa, /aaa (one or more 'a')
    // Using plus operator for "one or more"
    let path_validator = regex!(plus!('a'));

    let path_tests = vec![
        ("a", true, "single segment"),
        ("aa", true, "multiple segments"),
        ("aaa", true, "many segments"),
        ("", false, "no segments"),
    ];

    println!("Testing path validation (expecting one or more 'a'):");
    for (input, expected, description) in path_tests {
        let result = path_validator.run(input);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} '{}' -> {} ({})", status, input, result, description);
        assert_eq!(
            result, expected,
            "Failed for input '{}': {}",
            input, description
        );
    }

    println!("\n=== Optional Protocol Matcher ===\n");

    // Pattern for optional 'http' prefix: http?
    // Using question operator for "zero or one"
    let protocol_validator = regex!(question!('h'));

    let protocol_tests = vec![
        ("", true, "no protocol"),
        ("h", true, "with protocol"),
        ("hh", false, "double protocol"),
    ];

    println!("Testing optional protocol (expecting zero or one 'h'):");
    for (input, expected, description) in protocol_tests {
        let result = protocol_validator.run(input);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} '{}' -> {} ({})", status, input, result, description);
        assert_eq!(
            result, expected,
            "Failed for input '{}': {}",
            input, description
        );
    }

    println!("\n🎉 All real-world pattern tests passed!");
}
