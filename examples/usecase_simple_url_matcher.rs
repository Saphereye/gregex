extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Use Case: Simple URL Path Matcher ===\n");
    println!("This example shows pattern matching for URL-like paths.\n");

    // Pattern 1: Matching repeated path segments (one or more 'a')
    let path_pattern = regex!("a+");
    println!("Pattern 1: \"a+\" (one or more 'a's - like /a, /aa, /aaa)");

    let path_tests = vec![
        ("a", true, "single segment"),
        ("aa", true, "double segment"),
        ("aaa", true, "triple segment"),
        ("", false, "empty path"),
        ("b", false, "wrong character"),
    ];

    println!("\nTesting path pattern:");
    for (input, expected, description) in path_tests {
        let result = path_pattern.matches_exact(input);
        let status = if result == expected { "PASS" } else { "FAIL" };
        println!("[{}] '{}' -> {} ({})", status, input, result, description);
        assert_eq!(result, expected);
    }

    // Pattern 2: Optional protocol (like http or https)
    let protocol_pattern = regex!("h?");
    println!("\nPattern 2: \"h?\" (zero or one 'h' - like optional http prefix)");

    let protocol_tests = vec![
        ("", true, "no protocol"),
        ("h", true, "with protocol"),
        ("hh", false, "double protocol"),
    ];

    println!("\nTesting protocol pattern:");
    for (input, expected, description) in protocol_tests {
        let result = protocol_pattern.matches_exact(input);
        let status = if result == expected { "PASS" } else { "FAIL" };
        println!("[{}] '{}' -> {} ({})", status, input, result, description);
        assert_eq!(result, expected);
    }

    // Pattern 3: Complex path with alternation
    let complex_path = regex!("(a|b)+");
    println!("\nPattern 3: \"(a|b)+\" (one or more 'a' or 'b' - flexible paths)");

    let complex_tests = vec![
        ("a", true, "single a"),
        ("b", true, "single b"),
        ("ab", true, "a followed by b"),
        ("ba", true, "b followed by a"),
        ("aabbba", true, "mixed sequence"),
        ("", false, "empty"),
        ("c", false, "invalid character"),
    ];

    println!("\nTesting complex path pattern:");
    for (input, expected, description) in complex_tests {
        let result = complex_path.matches_exact(input);
        let status = if result == expected { "PASS" } else { "FAIL" };
        println!("[{}] '{}' -> {} ({})", status, input, result, description);
        assert_eq!(result, expected);
    }

    println!("\nAll URL path matching tests passed!");
}
